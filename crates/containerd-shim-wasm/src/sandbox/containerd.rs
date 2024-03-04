#![cfg(unix)]

use std::path::Path;

use containerd_client;
use containerd_client::services::v1::containers_client::ContainersClient;
use containerd_client::services::v1::content_client::ContentClient;
use containerd_client::services::v1::images_client::ImagesClient;
use containerd_client::services::v1::{GetContainerRequest, GetImageRequest, ReadContentRequest};
use containerd_client::tonic::transport::Channel;
use containerd_client::{tonic, with_namespace};
use futures::TryStreamExt;
use oci_spec::image::{Arch, ImageManifest, MediaType, Platform};
use tokio::runtime::Runtime;
use tonic::Request;

use crate::sandbox::error::{Error as ShimError, Result};
use crate::sandbox::oci::{self, WasmLayer};

pub(crate) struct Client {
    inner: Channel,
    rt: Runtime,
    namespace: String,
}

// sync wrapper implementation from https://tokio.rs/tokio/topics/bridging
impl Client {
    // wrapper around connection that will establish a connection and create a client
    pub fn connect(address: impl AsRef<Path>, namespace: impl ToString) -> Result<Client> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let inner = rt
            .block_on(containerd_client::connect(address))
            .map_err(|err| ShimError::Containerd(err.to_string()))?;

        Ok(Client {
            inner,
            rt,
            namespace: namespace.to_string(),
        })
    }

    // wrapper around read that will read the entire content file
    pub fn read_content(&self, digest: impl ToString) -> Result<Vec<u8>> {
        self.rt.block_on(async {
            let req = ReadContentRequest {
                digest: digest.to_string(),
                ..Default::default()
            };
            let req = with_namespace!(req, self.namespace);
            ContentClient::new(self.inner.clone())
                .read(req)
                .await
                .map_err(|err| ShimError::Containerd(err.to_string()))?
                .into_inner()
                .map_ok(|msg| msg.data)
                .try_concat()
                .await
                .map_err(|err| ShimError::Containerd(err.to_string()))
        })
    }

    pub fn get_image_content_sha(&self, image_name: impl ToString) -> Result<String> {
        self.rt.block_on(async {
            let name = image_name.to_string();
            let req = GetImageRequest { name };
            let req = with_namespace!(req, self.namespace);
            let digest = ImagesClient::new(self.inner.clone())
                .get(req)
                .await
                .map_err(|err| ShimError::Containerd(err.to_string()))?
                .into_inner()
                .image
                .ok_or_else(|| {
                    ShimError::Containerd(format!(
                        "failed to get image content sha for image {}",
                        image_name.to_string()
                    ))
                })?
                .target
                .ok_or_else(|| {
                    ShimError::Containerd(format!(
                        "failed to get image content sha for image {}",
                        image_name.to_string()
                    ))
                })?
                .digest;
            Ok(digest)
        })
    }

    pub fn get_image(&self, container_name: impl ToString) -> Result<String> {
        self.rt.block_on(async {
            let id = container_name.to_string();
            let req = GetContainerRequest { id };
            let req = with_namespace!(req, self.namespace);
            let image = ContainersClient::new(self.inner.clone())
                .get(req)
                .await
                .map_err(|err| ShimError::Containerd(err.to_string()))?
                .into_inner()
                .container
                .ok_or_else(|| {
                    ShimError::Containerd(format!(
                        "failed to get image for container {}",
                        container_name.to_string()
                    ))
                })?
                .image;
            Ok(image)
        })
    }

    // load module will query the containerd store to find an image that has an OS of type 'wasm'
    // If found it continues to parse the manifest and return the layers that contains the WASM modules
    // and possibly other configuration layers.
    pub fn load_modules(
        &self,
        containerd_id: impl ToString,
    ) -> Result<(Vec<oci::WasmLayer>, Platform)> {
        let image_name = self.get_image(containerd_id.to_string())?;
        let digest = self.get_image_content_sha(image_name)?;
        log::info!("RJ IS HERE image name {}, container id {:?}, digest {}", image_name.clone(), containerd_id.to_string().as_str(), digest.clone());
        let manifest = self.read_content(digest)?;
        let manifest = manifest.as_slice();
        let manifest = ImageManifest::from_reader(manifest)?;

        let image_config_descriptor = manifest.config();
        let image_config = self.read_content(image_config_descriptor.digest())?;
        let image_config = image_config.as_slice();

        // the only part we care about here is the platform values
        let platform: Platform = serde_json::from_slice(image_config)?;
        let Arch::Wasm = platform.architecture() else {
            log::info!("manifest is not in WASM OCI image format");
            return Ok((vec![], platform));
        };
        log::info!("found manifest with WASM OCI image format.");

        let layers = manifest
            .layers()
            .iter()
            .filter(|x| !is_image_layer_type(x.media_type()))
            .map(|config| {
                self.read_content(config.digest()).map(|module| WasmLayer {
                    config: config.clone(),
                    layer: module,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((layers, platform))
    }
}

fn is_image_layer_type(media_type: &MediaType) -> bool {
    match media_type {
        MediaType::ImageLayer
        | MediaType::ImageLayerGzip
        | MediaType::ImageLayerNonDistributable
        | MediaType::ImageLayerNonDistributableGzip
        | MediaType::ImageLayerNonDistributableZstd
        | MediaType::ImageLayerZstd => true,
        MediaType::Other(s)
            if s.as_str()
                .starts_with("application/vnd.docker.image.rootfs.") =>
        {
            true
        }
        _ => false,
    }
}

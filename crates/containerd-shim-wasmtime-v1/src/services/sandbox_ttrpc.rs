// This file is generated by ttrpc-compiler 0.5.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct ManagerClient {
    client: ::ttrpc::Client,
}

impl ManagerClient {
    pub fn new(client: ::ttrpc::Client) -> Self {
        ManagerClient {
            client: client,
        }
    }

    pub fn create(&self, ctx: ttrpc::context::Context, req: &super::sandbox::CreateRequest) -> ::ttrpc::Result<super::sandbox::CreateResponse> {
        let mut cres = super::sandbox::CreateResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "runwasi.services.sandbox.v1.Manager", "Create", cres);
        Ok(cres)
    }

    pub fn connect(&self, ctx: ttrpc::context::Context, req: &super::sandbox::ConnectRequest) -> ::ttrpc::Result<super::sandbox::ConnectResponse> {
        let mut cres = super::sandbox::ConnectResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "runwasi.services.sandbox.v1.Manager", "Connect", cres);
        Ok(cres)
    }
}

struct CreateMethod {
    service: Arc<std::boxed::Box<dyn Manager + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for CreateMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, sandbox, CreateRequest, create);
        Ok(())
    }
}

struct ConnectMethod {
    service: Arc<std::boxed::Box<dyn Manager + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ConnectMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, sandbox, ConnectRequest, connect);
        Ok(())
    }
}

pub trait Manager {
    fn create(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::sandbox::CreateRequest) -> ::ttrpc::Result<super::sandbox::CreateResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/runwasi.services.sandbox.v1.Manager/Create is not supported".to_string())))
    }
    fn connect(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::sandbox::ConnectRequest) -> ::ttrpc::Result<super::sandbox::ConnectResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/runwasi.services.sandbox.v1.Manager/Connect is not supported".to_string())))
    }
}

pub fn create_manager(service: Arc<std::boxed::Box<dyn Manager + Send + Sync>>) -> HashMap <String, Box<dyn ::ttrpc::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/runwasi.services.sandbox.v1.Manager/Create".to_string(),
                    std::boxed::Box::new(CreateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/runwasi.services.sandbox.v1.Manager/Connect".to_string(),
                    std::boxed::Box::new(ConnectMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods
}

use anyhow::Result;
use containerd_shim::cgroup::collect_metrics;
use containerd_shim::util::convert_to_any;
use protobuf::well_known_types::any::Any;
use log::info;

pub fn get_metrics(pid: u32) -> Result<Any> {
    info!("inside get metrics fn");
    let metrics = collect_metrics(pid)?;
    info!("after collect metrics inside get metrics fn");
    let metrics = convert_to_any(Box::new(metrics))?;
    Ok(metrics)
}

use anyhow::Result;
use containerd_shim::cgroup::collect_metrics;
use containerd_shim::util::convert_to_any;
use protobuf::well_known_types::any::Any;

use cgroups_rs::{
    cgroup::get_cgroups_relative_paths_by_pid, hierarchies, Cgroup, CgroupPid, MaxValue, Subsystem,
};

pub fn get_metrics(pid: u32) -> Result<Any> {
    let metrics = collect_metrics(pid)?;

    let metrics = convert_to_any(Box::new(metrics))?;
    Ok(metrics)
}

// get_cgroup will return either cgroup v1 or v2 depending on system configuration
fn get_cgroup(pid: u32) -> Result<Cgroup> {
    let hierarchies = hierarchies::auto();
    let cgroup = if hierarchies.v2() {
        let path = get_cgroups_v2_path_by_pid(pid)?;
        Cgroup::load(hierarchies, path.as_str())
    } else {
        // get container main process cgroup
        let path = get_cgroups_relative_paths_by_pid(pid)
            .map_err(other_error!(e, "get process cgroup"))?;
        Cgroup::load_with_relative_paths(hierarchies::auto(), Path::new("."), path)
    };
    Ok(cgroup)
}

/// Get the cgroups v2 path given a PID
pub fn get_cgroups_v2_path_by_pid(pid: u32) -> Result<String> {
    // todo: should upstream to cgroups-rs
    let path = format!("/proc/{}/cgroup", pid);
    let content = fs::read_to_string(path).map_err(io_error!(e, "read cgroup"))?;
    let content = content.trim_end_matches('\n');

    parse_cgroups_v2_path(content)
}

// https://github.com/opencontainers/runc/blob/1950892f69597aa844cbf000fbdf77610dda3a44/libcontainer/cgroups/fs2/defaultpath.go#L83
fn parse_cgroups_v2_path(content: &str) -> std::prelude::v1::Result<String, Error> {
    // the entry for cgroup v2 is always in the format like `0::$PATH`
    // where 0 is the hierarchy ID, the controller name is ommit in cgroup v2
    // and $PATH is the cgroup path
    // see https://docs.kernel.org/admin-guide/cgroup-v2.html
    let parts: Vec<&str> = content.splitn(3, ":").collect();

    if parts.len() < 3 {
        return Err(Error::Other(format!("invalid cgroup path: {}", content)));
    }

    if parts[0] == "0" && parts[1].is_empty() {
        // Check if parts[2] starts with '/', remove it if present.
        let path = parts[2].strip_prefix('/').unwrap_or(parts[2]);
        return Ok(format!("/sys/fs/cgroup/{}", path));
    }

    Err(Error::Other("cgroup path not found".into()))
}
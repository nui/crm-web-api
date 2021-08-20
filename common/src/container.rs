use std::path::PathBuf;

fn is_mac() -> bool {
    cfg!(target_os = "macos")
}

struct CGroup<'a> {
    #[allow(dead_code)]
    hierarchy_id: &'a str,
    #[allow(dead_code)]
    subsystems: &'a str,
    control_group: &'a str,
}

impl<'a> CGroup<'a> {
    pub fn parse(line: &'a str) -> Option<Self> {
        let mut iter = line.split(':');
        Some(Self {
            hierarchy_id: iter.next()?,
            subsystems: iter.next()?,
            control_group: iter.next()?,
        })
    }

    pub fn is_container(&self) -> bool {
        let cgroup = self.control_group;
        cgroup.starts_with("/docker") || cgroup.starts_with("/kube")
    }
}

/// Test if any input line is match known pattern of docker container
///
/// Unrecognized line is silently ignored
fn is_container(s: &str) -> bool {
    s.lines()
        .flat_map(CGroup::parse)
        .any(|cg| cg.is_container())
}

/// Test if this process is containerized
pub fn is_containerized() -> bool {
    if is_mac() {
        return false;
    }
    let cgroup = PathBuf::from(format!("/proc/{}/cgroup", std::process::id()));
    // It is fine to panic here since we expect `/proc/<PID>/cgroup` to be readable
    let contents = std::fs::read_to_string(&cgroup).expect("Cannot open cgroup file");
    is_container(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_parse() {
        let actual = CGroup::parse("12:cpu,cpuacct:/").unwrap();
        let expect = CGroup {
            hierarchy_id: "12",
            subsystems: "cpu,cpuacct",
            control_group: "/",
        };
        assert_eq!(actual.hierarchy_id, expect.hierarchy_id);
        assert_eq!(actual.subsystems, expect.subsystems);
        assert_eq!(actual.control_group, expect.control_group);
    }

    #[test]
    fn test_is_container() {
        let docker_cgroup = r#"
12:cpu,cpuacct:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
11:perf_event:/docker/c6fa62a9938149f6098fd0cdaffc9cdf0f526f25d97b5f6e2a4cc1fccc7f7ce1
10:rdma:/"#;
        assert!(is_container(docker_cgroup));
        let init_cgroup = r#"
12:cpu,cpuacct:/
11:perf_event:/
0::/init.scope"#;
        assert!(!is_container(init_cgroup));
        let k8s_cgroup = r#"
12:hugetlb:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1
11:cpuset:/kubepods/besteffort/poda00e29fd-7bbd-11e9-8679-fa163ea7e3b8/c4b1403f3d9c7ce261be851df71d9a9773c53419075ccda39ae8fe6a39fd2eb1"#;
        assert!(is_container(k8s_cgroup));
    }
}

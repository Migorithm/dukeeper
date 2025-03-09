use std::collections::HashMap;

use super::{lease::Lease, node::NodeGroup};

#[derive(Default)]
pub struct ConsistentCore {
    nodes: HashMap<Lease, NodeGroup>,
}

impl ConsistentCore {
    pub(crate) fn register_lease(&mut self, lease: Lease, node_id: u64) -> Result<(), String> {
        let nodes = self.nodes.entry(lease).or_insert(Default::default());
        nodes.add_controller(node_id)?;

        Ok(())
    }

    pub(crate) fn watch(&mut self, group_id: &str, node_id: u64) -> Result<(), String> {
        let nodes = self.nodes.get_mut(group_id).ok_or("Group not found")?;
        nodes.add_watcher(node_id)?;
        Ok(())
    }
}

#[test]
fn test_register_lease() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    assert!(core.register_lease(lease, 1).is_ok());
}

#[test]
fn test_register_lease_twice_return_err() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    assert!(core.register_lease(lease.clone(), 1).is_ok());
    assert!(core.register_lease(lease.clone(), 2).is_err());
}

#[test]
fn test_watch() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    assert!(core.register_lease(lease.clone(), 1).is_ok());
    assert!(core.watch("group1", 2).is_ok());
}

#[test]
fn test_watch_twice_return_err() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    assert!(core.register_lease(lease.clone(), 1).is_ok());
    assert!(core.watch("group1", 2).is_ok());
    assert!(core.watch("group1", 2).is_err());
}

#[test]
fn test_watch_without_controller_return_err() {
    let mut core = ConsistentCore::default();
    assert!(core.watch("group1", 2).is_err());
}

fn current_time_in_sec() -> u64 {
    let sys_now = std::time::SystemTime::now();

    // convert sys_now to u64
    sys_now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

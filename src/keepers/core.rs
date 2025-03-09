use std::collections::HashMap;

use crate::application_servers::ServerCommand;

use super::{lease::Lease, node::NodeGroup};

#[derive(Default)]
pub struct ConsistentCore {
    nodes: HashMap<Lease, NodeGroup>,
}

impl ConsistentCore {
    pub(crate) fn register_lease(
        &mut self,
        lease: Lease,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerCommand>,
    ) -> Result<(), String> {
        let nodes = self.nodes.entry(lease).or_insert(Default::default());
        nodes.add_controller(node_id, tx)?;

        Ok(())
    }

    pub(crate) fn watch(
        &mut self,
        group_id: &str,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerCommand>,
    ) -> Result<(), String> {
        let nodes = self.nodes.get_mut(group_id).ok_or("Group not found")?;
        nodes.add_watcher(node_id, tx)?;
        Ok(())
    }
}

pub enum CoreCommand {
    RegisterLease {
        lease: Lease,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerCommand>,
    },
    Watch {
        group_id: String,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerCommand>,
    },
}

#[test]
fn test_register_lease() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    let (tx, _) = std::sync::mpsc::channel();
    assert!(core.register_lease(lease, 1, tx).is_ok());
}

#[test]
fn test_register_lease_twice_return_err() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    let (tx, _) = std::sync::mpsc::channel();
    assert!(core.register_lease(lease.clone(), 1, tx.clone()).is_ok());
    assert!(core.register_lease(lease.clone(), 2, tx).is_err());
}

#[test]
fn test_watch() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    let (tx, _) = std::sync::mpsc::channel();
    assert!(core.register_lease(lease.clone(), 1, tx).is_ok());

    let (follower_tx, _) = std::sync::mpsc::channel();
    assert!(core.watch("group1", 2, follower_tx).is_ok());
}

#[test]
fn test_watch_twice_return_err() {
    let mut core = ConsistentCore::default();

    let lease = Lease::new("group1", 300);
    let (tx, _) = std::sync::mpsc::channel();
    assert!(core.register_lease(lease.clone(), 1, tx).is_ok());

    let (follower_tx, _) = std::sync::mpsc::channel();
    assert!(core.watch("group1", 2, follower_tx.clone()).is_ok());
    assert!(core.watch("group1", 2, follower_tx).is_err());
}

#[test]
fn test_watch_without_controller_return_err() {
    let mut core = ConsistentCore::default();
    let (tx, _) = std::sync::mpsc::channel();
    assert!(core.watch("group1", 2, tx).is_err());
}

fn current_time_in_sec() -> u64 {
    let sys_now = std::time::SystemTime::now();

    // convert sys_now to u64
    sys_now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

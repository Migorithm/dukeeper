use std::collections::HashMap;

use crate::application_servers::ServerInbox;

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
        tx: std::sync::mpsc::Sender<ServerInbox>,
    ) -> Result<(), String> {
        let nodes = self.nodes.entry(lease).or_insert(Default::default());
        nodes.add_controller(node_id, tx)?;

        Ok(())
    }

    pub(crate) fn watch(
        &mut self,
        group_id: &str,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerInbox>,
    ) -> Result<(), String> {
        let nodes = self.nodes.get_mut(group_id).ok_or("Group not found")?;
        nodes.add_watcher(node_id, tx)?;
        Ok(())
    }

    pub(crate) fn run(mut self) -> std::sync::mpsc::Sender<CoreCommand> {
        let (core_sender, recv) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            for cmd in recv {
                match cmd {
                    CoreCommand::RegisterLease { lease, node_id, tx } => {
                        if let Err(e) = self.register_lease(lease.clone(), node_id, tx.clone()) {
                            tx.send(ServerInbox::Result(format!("Error: {}", e)))
                                .unwrap();
                        } else {
                            tx.send(ServerInbox::Result(format!(
                                "You became a controller for {}",
                                lease.group_id
                            )))
                            .unwrap()
                        }
                    }
                    CoreCommand::Watch {
                        group_id,
                        node_id,
                        tx,
                    } => {
                        if let Err(e) = self.watch(&group_id, node_id, tx.clone()) {
                            tx.send(ServerInbox::Result(format!("Error: {}", e)))
                                .unwrap();
                        } else {
                            tx.send(ServerInbox::Result("Ok".to_string())).unwrap()
                        }
                    }
                    CoreCommand::CheckLease => {
                        let now = current_time_in_sec();
                        for (lease, nodes) in self.nodes.iter_mut() {
                            if lease.ttl < now {
                                nodes.expire_controller();
                            }
                        }
                    }
                }
            }
        });

        std::thread::spawn({
            let sd = core_sender.clone();
            move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    sd.send(CoreCommand::CheckLease).unwrap()
                }
            }
        });
        core_sender
    }
}

pub enum CoreCommand {
    RegisterLease {
        lease: Lease,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerInbox>,
    },
    Watch {
        group_id: String,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerInbox>,
    },
    CheckLease,
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

pub fn current_time_in_sec() -> u64 {
    let sys_now = std::time::SystemTime::now();

    // convert sys_now to u64
    sys_now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

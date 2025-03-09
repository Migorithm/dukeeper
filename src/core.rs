use std::collections::HashMap;

use crate::node::Lease;

#[derive(Default)]
pub struct ConsistentCore {
    nodes: HashMap<Lease, Vec<Node>>,
}

impl ConsistentCore {
    pub(crate) fn register_lease(&mut self, lease: Lease, node_id: u64) -> Result<(), String> {
        let nodes = self.nodes.entry(lease).or_insert(vec![]);

        nodes.push(Node {
            id: node_id,
            role: Role::Controller,
        });

        if nodes.iter().filter(|n| n.role == Role::Controller).count() > 1 {
            return Err("Controller already exists".to_string());
        }
        Ok(())
    }

    pub(crate) fn watch(&mut self, group_id: &str, node_id: u64) -> Result<(), String> {
        let nodes = self.nodes.get_mut(group_id).ok_or("Group not found")?;
        nodes.push(Node {
            id: node_id,
            role: Role::Follower,
        });
        Ok(())
    }
}

#[derive(Debug)]
pub struct Node {
    id: u64,
    role: Role,
}

#[derive(Debug, PartialEq)]
enum Role {
    Controller,
    Follower,
}

#[test]
fn test_register_lease() {
    let mut core = ConsistentCore::default();

    let group_id = Lease::new("group1", 300);
    assert!(core.register_lease(group_id, 1).is_ok());
}

#[test]
fn test_register_lease_twice_return_err() {
    let mut core = ConsistentCore::default();

    let group_id = Lease::new("group1", 300);
    assert!(core.register_lease(group_id.clone(), 1).is_ok());
    assert!(core.register_lease(group_id.clone(), 2).is_err());
}

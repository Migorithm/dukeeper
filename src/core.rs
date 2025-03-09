use std::collections::HashMap;

#[derive(Default)]
pub struct ConsistentCore {
    nodes: HashMap<GroupId, Vec<Node>>,
}

impl ConsistentCore {
    pub(crate) fn register_lease(
        &mut self,
        group_id: GroupId,
        node_id: u64,
        ttl: u64,
    ) -> Result<(), String> {
        let nodes = self.nodes.entry(group_id).or_insert(vec![]);

        nodes.push(Node {
            id: node_id,
            role: Role::Controller,
            ttl,
        });

        if nodes.iter().filter(|n| n.role == Role::Controller).count() > 1 {
            return Err("Controller already exists".to_string());
        }

        Ok(())
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct GroupId(String);

#[derive(Debug)]
pub struct Node {
    id: u64,
    role: Role,
    ttl: u64,
}

#[derive(Debug, PartialEq)]
enum Role {
    Controller,
    Follower,
}

#[test]
fn test_register_lease() {
    let mut core = ConsistentCore::default();

    let group_id = GroupId("group1".to_string());
    assert!(core.register_lease(group_id, 1, 10).is_ok());
}

#[test]
fn test_register_lease_twice_return_err() {
    let mut core = ConsistentCore::default();

    let group_id = GroupId("group1".to_string());
    assert!(core.register_lease(group_id.clone(), 1, 10).is_ok());
    assert!(core.register_lease(group_id.clone(), 2, 10).is_err());
}

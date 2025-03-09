use std::collections::HashMap;

#[derive(Default)]
pub struct ConsistentCore {
    nodes: HashMap<GroupId, Vec<Node>>,
}

impl ConsistentCore {
    pub(crate) fn register(&mut self, group_id: GroupId, mut node_id: u64, ttl: u64) {
        let nodes = self.nodes.entry(group_id).or_insert(vec![]);

        nodes.push(Node {
            id: node_id,
            role: if nodes.is_empty() {
                Role::Controller
            } else {
                Role::Follower
            },
            ttl,
        });
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct GroupId(String);

pub struct Node {
    id: u64,
    role: Role,
    ttl: u64,
}

enum Role {
    Controller,
    Follower,
}

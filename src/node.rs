#[derive(Debug, Default)]
pub struct NodeGroup {
    nodes: Vec<Node>,
}

impl NodeGroup {
    pub fn add_controller(&mut self, node_id: u64) -> Result<(), String> {
        self.nodes.push(Node {
            id: node_id,
            role: Role::Controller,
        });

        if self
            .nodes
            .iter()
            .filter(|n| n.role == Role::Controller)
            .count()
            > 1
        {
            return Err("Controller already exists".to_string());
        }
        Ok(())
    }

    pub fn add_watcher(&mut self, node_id: u64) -> Result<(), String> {
        if self.nodes.iter().find(|n| n.id == node_id).is_some() {
            return Err("Node already exists".to_string());
        }
        self.nodes.push(Node {
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

impl Node {}

#[derive(Debug, PartialEq)]
pub enum Role {
    Controller,
    Follower,
}

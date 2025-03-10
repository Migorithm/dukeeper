use crate::application_servers::ServerInbox;

#[derive(Debug, Default)]
pub struct NodeGroup {
    nodes: Vec<Node>,
}

impl NodeGroup {
    pub fn add_controller(
        &mut self,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerInbox>,
    ) -> Result<(), String> {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.role = Role::Controller;
        } else {
            self.nodes.push(Node {
                id: node_id,
                role: Role::Controller,
                tx,
            })
        };

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

    pub fn add_watcher(
        &mut self,
        node_id: u64,
        tx: std::sync::mpsc::Sender<ServerInbox>,
    ) -> Result<(), String> {
        if self.nodes.iter().find(|n| n.id == node_id).is_some() {
            return Err("Node already exists".to_string());
        }
        self.nodes.push(Node {
            id: node_id,
            role: Role::Follower,
            tx,
        });

        Ok(())
    }

    pub(crate) fn expire_controller(&mut self) {
        self.nodes.retain(|n| n.role == Role::Follower);
        for node in self.nodes.iter_mut() {
            node.tx.send(ServerInbox::ControllerExpired).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct Node {
    id: u64,
    role: Role,
    tx: std::sync::mpsc::Sender<ServerInbox>,
}

impl Node {}

#[derive(Debug, PartialEq)]
pub enum Role {
    Controller,
    Follower,
}

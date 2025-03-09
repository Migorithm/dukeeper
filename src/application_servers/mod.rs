// simulate a servers

use crate::keepers::core::CoreCommand;

pub struct Server {
    id: u64,
    recv: std::sync::mpsc::Receiver<ServerInbox>,
    tx: std::sync::mpsc::Sender<CoreCommand>,
    self_tx: std::sync::mpsc::Sender<ServerInbox>,
}

impl Server {
    pub fn new(id: u64, tx: std::sync::mpsc::Sender<CoreCommand>) -> Self {
        let (self_tx, recv) = std::sync::mpsc::channel();
        Server {
            id,
            recv,
            tx,
            self_tx,
        }
    }

    pub fn register_lease(&self, group_id: &str, ttl: u64) -> String {
        let lease = crate::keepers::lease::Lease::new(group_id, ttl);
        self.tx
            .send(CoreCommand::RegisterLease {
                lease,
                node_id: self.id,
                tx: self.self_tx.clone(),
            })
            .unwrap();

        let ServerInbox::Result(s) = self.recv.recv().unwrap() else {
            panic!()
        };
        s
    }

    pub fn watch(&self, group_id: &str) -> String {
        self.tx
            .send(CoreCommand::Watch {
                group_id: group_id.into(),
                node_id: self.id,
                tx: self.self_tx.clone(),
            })
            .unwrap();

        let ServerInbox::Result(s) = self.recv.recv().unwrap() else {
            panic!()
        };
        s
    }
}

pub enum ServerInbox {
    ControllerOut,
    Result(String),
}

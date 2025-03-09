// simulate a servers

use crate::keepers::core::CoreCommand;

struct Server {
    id: u64,
    recv: std::sync::mpsc::Receiver<ServerCommand>,
    tx: std::sync::mpsc::Sender<CoreCommand>,
}

pub enum ServerCommand {
    ControllerOut,
}

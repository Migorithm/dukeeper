pub mod application_servers;
pub mod keepers;

#[cfg(test)]
mod test {

    use std::{
        sync::{Arc, Mutex},
        thread::sleep,
        time::Duration,
    };

    use crate::{
        application_servers::Server,
        keepers::core::{ConsistentCore, current_time_in_sec},
    };

    use super::*;

    fn logger() -> Arc<Mutex<Vec<String>>> {
        Arc::new(Mutex::new(Vec::new()))
    }
    #[test]
    fn test_register_lease() {
        let core = keepers::core::ConsistentCore::default();
        let core_sender = core.run();

        let controller = Server::new(1, core_sender);

        //WHEN
        let result = controller.register_lease("migo_group", 300);

        //THEN
        assert_eq!("You became a controller for migo_group", result);
    }

    #[test]
    fn test_register_lease_twice() {
        let core = keepers::core::ConsistentCore::default();
        let core = core;
        let core_sender = core.run();

        let controller = Server::new(1, core_sender.clone());

        //WHEN
        let _ = controller.register_lease("migo_group", 300);

        let second = Server::new(2, core_sender);
        let result = second.register_lease("migo_group", 300);

        //THEN
        assert_eq!("Error: Controller already exists", result);
    }

    #[test]
    fn test_watcher_notified_when_controller_gone() {
        let core = ConsistentCore::default();
        let core_sender = core.run();

        let controller = Server::new(1, core_sender.clone());

        let current_time_in_secs = current_time_in_sec() + 1;
        let _ = controller.register_lease("migo_group", current_time_in_secs);

        let follower = Server::new(2, core_sender.clone());
        let logger = logger();
        std::thread::spawn({
            let logger = logger.clone();
            move || follower.watch("migo_group", logger)
        });

        //WHEN
        drop(controller);
        sleep(Duration::from_secs(2));

        //THEN
        let logger = logger.lock().unwrap();
        assert_eq!(3, logger.len());
        assert_eq!("Ok", logger[0]);
        assert_eq!("Controller is gone", logger[1]);
        assert_eq!("You became a controller for migo_group", logger[2]);
    }
}

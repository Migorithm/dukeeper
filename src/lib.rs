pub mod application_servers;
pub mod keepers;

#[cfg(test)]
mod test {
    use std::{
        sync::{Arc, Mutex},
        thread::sleep,
        time::Duration,
    };

    use super::*;

    fn logger() -> Arc<Mutex<Vec<String>>> {
        Arc::new(Mutex::new(Vec::new()))
    }
    #[test]
    fn test_register_lease() {
        let (core_sender, rx) = std::sync::mpsc::channel();
        let core = keepers::core::ConsistentCore::default();
        let core = core;
        std::thread::spawn(|| core.run(rx));

        let server = application_servers::Server::new(1, core_sender);

        //WHEN
        let result = server.register_lease("migo_group", 300);

        //THEN
        assert_eq!("Ok", result);
    }

    #[test]
    fn test_register_lease_twice() {
        let (core_sender, rx) = std::sync::mpsc::channel();
        let core = keepers::core::ConsistentCore::default();
        let core = core;
        std::thread::spawn(|| core.run(rx));

        let server = application_servers::Server::new(1, core_sender);

        //WHEN
        let _ = server.register_lease("migo_group", 300);
        let result = server.register_lease("migo_group", 300);

        //THEN
        assert_eq!("Error: Controller already exists", result);
    }

    #[test]
    fn test_watch() {
        let (core_sender, rx) = std::sync::mpsc::channel();
        let core = keepers::core::ConsistentCore::default();
        let core = core;
        std::thread::spawn(|| core.run(rx));

        let server = application_servers::Server::new(1, core_sender.clone());
        let _ = server.register_lease("migo_group", 300);

        //WHEN
        let follower = application_servers::Server::new(2, core_sender);
        let logger = logger();
        std::thread::spawn({
            let logger = logger.clone();
            move || follower.watch("migo_group", logger)
        });

        //THEN
        sleep(Duration::from_millis(500));
        let logger = logger.lock().unwrap();
        assert_eq!(1, logger.len());

        assert_eq!("Ok", logger[0]);
    }

    #[test]
    fn test_watcher_notified_when_controller_gone() {
        let (core_sender, rx) = std::sync::mpsc::channel();
        let core = keepers::core::ConsistentCore::default();
        let core = core;
        std::thread::spawn(|| core.run(rx));

        let server = application_servers::Server::new(1, core_sender.clone());
        let _ = server.register_lease("migo_group", 300);

        let follower = application_servers::Server::new(2, core_sender.clone());
        let logger = logger();
        std::thread::spawn({
            let logger = logger.clone();
            move || follower.watch("migo_group", logger)
        });

        //WHEN
        let _ = server.stop();

        //THEN
    }
}

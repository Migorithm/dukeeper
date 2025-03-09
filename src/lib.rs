pub mod application_servers;
pub mod keepers;

#[cfg(test)]
mod test {
    use super::*;

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
}

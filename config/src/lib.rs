pub mod server;

use server::Server;
#[derive(Debug)]
pub struct Config {
    pub server: Server,
}

impl Config {
    pub fn default() -> Config {
        Config {
            server: Server::default(),
        }
    }
}

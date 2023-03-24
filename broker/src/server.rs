use std::error::Error;
use listener::listener::{Listeners, Listener};
#[derive(Debug)]
pub struct Server {
    pub config: config::Config,
    pub listeners: Listeners,
}

impl Server {
    pub fn new(config: config::Config) -> Server {
        Server { config, listeners: Listeners::new() }
    }

    pub fn add_listener(&mut self, name: String, l: Box<dyn Listener>) -> Result<(), Box<dyn Error>> {
       self.listeners.add(name, l)
    }

    pub fn add_hook() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn serve(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

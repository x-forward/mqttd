use std::error::Error;

#[derive(Debug)]
pub struct Server {
    pub config: config::Config,
}

impl Server {
    pub fn new(config: config::Config) -> Server {
        Server { config }
    }

    pub fn add_listener() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn add_hook() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn serve(&self) ->  Result<(), Box<dyn Error>> {
        Ok(())
    }
}

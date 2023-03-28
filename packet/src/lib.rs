use std::error::Error;

use codec::frame::Frame;
#[derive(Debug)]
pub struct Connect {}

#[derive(Debug)]
pub enum Command {
    Connect(Connect),
}

impl Command {
    pub fn from_frame(_frame: Frame) -> Result<Command, Box<dyn Error>> {
        todo!()
    }

    pub async fn apply() -> Result<Command, Box<dyn Error>> {
        todo!()
    }
}

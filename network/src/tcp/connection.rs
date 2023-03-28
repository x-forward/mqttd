use codec::frame::Frame;

use std::error::Error;
use tokio::net::TcpStream;
pub struct Connection {
    pub socket: TcpStream,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection { socket }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Box<dyn Error>> {
        todo!()
    }

    pub async fn parse_frame(&mut self) -> Result<Option<Frame>, Box<dyn Error>> {
        todo!()
    }

    pub async fn write_frame(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

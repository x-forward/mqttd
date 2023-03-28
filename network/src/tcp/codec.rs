use crate::tcp::connection::Connection;
use crate::tcp::shutdown::Shutdown;
use codec::frame::Error;
use packet::Command;
use tracing::{debug, info, instrument};
pub struct Codec {
    pub connection: Connection,
    pub shutdown: Shutdown,
}

impl Codec {
    pub fn new(connection: Connection, shutdown: Shutdown) -> Codec {
        Codec {
            connection: connection,
            shutdown: shutdown,
        }
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<(), Error> {
        while !self.shutdown.is_shutdown() {
            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res.unwrap(),
                _ = self.shutdown.recv() => {
                    return  Ok(());
                }
            };

            info!("reading frame {:?}", maybe_frame);
            let frame = match maybe_frame {
                Some(frame) => frame,
                None => return Ok(()),
            };

            let cmd = Command::from_frame(frame).unwrap();
            debug!(?cmd);
            todo!()
        }
        Ok(())
    }
}

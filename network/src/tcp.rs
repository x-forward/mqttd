// use crate::codec::Codec;
use crate::codec::Codec;
use crate::connection::Connection;
use crate::shutdown::Shutdown;
use std::error::Error;
// use std::future::Future;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Semaphore};
use tracing::{error, info};

pub struct Listener {
    pub listener: TcpListener,
    pub limit_connections: Arc<Semaphore>,
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl Listener {
    pub fn new(
        listener: TcpListener,
        notify_shutdown: broadcast::Sender<()>,
        shutdown_complete_tx: mpsc::Sender<()>,
        limit_connections: Arc<Semaphore>,
    ) -> Listener {
        Listener {
            listener,
            limit_connections: limit_connections,
            notify_shutdown,
            shutdown_complete_tx,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        info!("accepting inbound connections");
        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();
            let socket = self.accept().await?;

            let mut codec = Codec {
                connection: Connection::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                // _shutdown_complete: self.shutdown_complete_tx.clone(),
            };
            tokio::spawn(async move {
                if let Err(err) = codec.run().await {
                    error!(cause = ?err, "connection error");
                }
                drop(permit)
            });
        }
    }

    pub async fn accept(&mut self) -> Result<TcpStream, Box<dyn Error>> {
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

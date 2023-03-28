use crate::tcp::codec;
use crate::tcp::connection::Connection;
use crate::tcp::shutdown::Shutdown;
use std::error::Error;
use std::future::Future;
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

pub async fn run(
    listener: TcpListener,
    shutdown: impl Future,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: &mut mpsc::Receiver<()>,
) {
    let mut tcp_server = new(
        listener,
        notify_shutdown,
        shutdown_complete_tx,
        Arc::new(Semaphore::new(0x7A)),
    );

    tokio::select! {
        res = tcp_server.run() => {
            if let Err(err) = res {
                error!(cause = %err, "failed to accept");
            }
        }
        _ = shutdown => {
            info!("shutting down");
        }
    }

    let Listener {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = tcp_server;
    drop(notify_shutdown);
    drop(shutdown_complete_tx);
    let _ = shutdown_complete_rx.recv().await;
}

impl Listener {
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

            let mut c = codec::new(
                Connection::new(socket),
                Shutdown::new(self.notify_shutdown.subscribe()),
            );

            tokio::spawn(async move {
                if let Err(err) = c.run().await {
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

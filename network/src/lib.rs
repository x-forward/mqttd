pub mod tcp;

#[cfg(test)]
mod tests {
    use crate::tcp::listener;
    use tokio::net::TcpListener;
    use tokio::sync::{broadcast, mpsc};
    #[tokio::test]
    async fn it_works() {
        tracing_subscriber::fmt::init();
        let (notify_shutdown, _) = broadcast::channel(1);
        let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);
        let listener = TcpListener::bind("127.0.0.1:1883").await.unwrap();
        listener::run(
            listener,
            tokio::signal::ctrl_c(),
            notify_shutdown,
            shutdown_complete_tx,
            &mut shutdown_complete_rx,
        )
        .await;
    }
}

pub mod tcp;

#[cfg(test)]
mod tests {
    use crate::tcp::listener;
    use tokio::net::TcpListener;
    #[tokio::test]
    async fn it_works() {
        tracing_subscriber::fmt::init();
        let listener = TcpListener::bind("127.0.0.1:1883").await.unwrap();
        listener::run(listener, tokio::signal::ctrl_c()).await;
    }
}

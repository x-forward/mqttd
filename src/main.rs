use broker::server::Server;
use config::Config;

#[tokio::main]
async fn main() {
    let s = Server::new(Config::default());
    s.serve().await.unwrap()
}

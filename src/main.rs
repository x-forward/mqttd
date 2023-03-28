use broker::server::Server;
use config::Config;

#[tokio::main]
async fn main() {
    let s = Server::new(Config::default());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

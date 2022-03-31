use server::Server;

mod server;

fn main() {
  Server::new("127.0.0.1:8080".to_string()).run();
}


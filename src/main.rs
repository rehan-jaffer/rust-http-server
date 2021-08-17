mod server;
mod linescodec;
mod http;

fn main() {

  let listen_address = "0.0.0.0";
  let port = 8009;

  let mut server = server::Server::new(listen_address, port, true);
  server.start();

}

mod server;
mod linescodec;
mod http;
mod cmd;
use std::env;

fn main() {

  let args: Vec<String> = env::args().collect();
  let mut parser = cmd::parser::CommandLineParser::new(args);
  let opts = parser.parse();

  let mut server = server::Server::new(opts);
  server.start();

}

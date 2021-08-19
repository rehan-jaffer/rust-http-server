use std::net::TcpListener;
use std::net::TcpStream;
use threadpool::ThreadPool;
use std::sync::{Arc};

mod debugger;
mod plugins;

use super::linescodec;

pub struct Server<'a> {
  pub listen_address: &'a str,
  pub port: u32,
  pub debug_mode: bool,
  pub flags: super::cmd::opts::Flags,
  pool: ThreadPool,
  debugger: debugger::Debugger
}

impl<'a> Server<'a> {

   pub fn new(opts : super::cmd::opts::CommandLineOpts<'a>) -> Self {
    
    let debug_mode_enabled = opts.flags.contains(super::cmd::opts::FlagType::DebugMode);

    Self { 
       listen_address: opts.listen_address,
       port: opts.port,
       debug_mode: debug_mode_enabled,
       flags: opts.flags,
       debugger: debugger::Debugger { enabled: debug_mode_enabled },
       pool: ThreadPool::new(4),
      }
   }

   fn display_options(&self) -> String {

    let mut line = String::new();
    line += "Starting with options: ";

    for flag in self.flags.list() {
       line += format!("{:?}", flag).as_str();
    }

    return line;

   }

   pub fn start(&mut self) {

      self.debugger.debug_line("Rust HTTP Server [v0.1]");
      self.debugger.debug_line(self.display_options().as_str());
      let listener = TcpListener::bind(self.listen_string());
      self.debugger.debug_line(&format!("Listening on {}", self.listen_string().as_str()));

      match listener {
          Ok(connection) => {
            self.debugger.debug_line("Accepting connections...");
            for stream in connection.incoming() {
              let stream = stream.unwrap();
              self.handle(stream);
            }

          }
          Err(err) => {
              println!("Error {}!", err);
          }
      }

    }

    fn listen_string(&self) -> String {

      return format!("{}:{}", self.listen_address, self.port)

    }

    fn handle(&mut self, stream: TcpStream) {
      
      let arc_stream = Arc::new(stream);

      self.pool.execute(move || {

        let mut handler = RequestHandler { 
          stream: &mut arc_stream.try_clone().unwrap(), 
          debugger: debugger::Debugger { enabled: true },
          codec: linescodec::LinesCodec::new(arc_stream.try_clone().unwrap()).unwrap()
        };
        handler.handle();
      });

      return ();

  }
}

struct RequestHandler<'a> {
  stream: &'a mut TcpStream,
  debugger: debugger::Debugger,
  codec: linescodec::LinesCodec
}

impl<'a> RequestHandler<'a> {

  fn get_parsed_request(&mut self, message: Vec<String>) -> super::http::request::HTTPRequest {

    let mut request_parser = super::http::parsers::HTTPRequestParser { request: message };
    return request_parser.parse().unwrap();
  }

  pub fn handle(&mut self) {

    let message = self.codec.get_lines().unwrap();
    let request = self.get_parsed_request(message);
    let response = plugins::file::File::execute(&request);

    self.debugger.debug_request(request, response.clone(), self.stream.peer_addr().unwrap());

    self.codec.send(response);

    return ();
  }
}



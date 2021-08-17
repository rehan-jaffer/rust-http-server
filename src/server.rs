use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fmt::{Display, Formatter, Result};
use std::thread;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
mod debugger;

use super::linescodec;

pub struct Server<'a> {
  pub listen_address: &'a str,
  pub port: u32,
  pub debug_mode: bool,
  pool: ThreadPool,
  debugger: debugger::Debugger
}

impl<'a> Server<'a> {
   pub fn new(listen_addr: &'a str, port: u32, debug: bool) -> Self {
     Self { 
       listen_address: listen_addr,
       port: port,
       debug_mode: debug, 
       debugger: debugger::Debugger { enabled: debug },
       pool: ThreadPool::new(4)
      }
   }
   pub fn start(&mut self) {

      self.debugger.debug_line("Starting server");
      self.debugger.debug_line(self.listen_string().as_str());

      let listener = TcpListener::bind(self.listen_string());

      match listener {
          Ok(connection) => {
            for stream in connection.incoming() {
              let mut stream = stream.unwrap();
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
          debugger: debugger::Debugger { enabled: true } 
        };
        handler.handle();
      });

      return ();

  }
}

struct RequestHandler<'a> {
  stream: &'a mut TcpStream,
  debugger: debugger::Debugger
}

impl<'a> RequestHandler<'a> {
  pub fn handle(&mut self) {

    let mut lines_codec = linescodec::LinesCodec::new(self.stream.try_clone().unwrap()).unwrap();
    let message = lines_codec.get_lines().unwrap();

    let mut request_parser = super::http::parsers::HTTPRequestParser { request: message };
    let request = request_parser.parse().unwrap();

    let ip = self.stream.peer_addr().unwrap();

    let response = super::http::response::HTTPResponse { 
      status: super::http::response::HTTPResponseType::Ok,
      content: "hi".to_string()
    };

    self.debugger.debug_request(request, response.clone(), ip);

    lines_codec.send(response);

    return ();
  }
}


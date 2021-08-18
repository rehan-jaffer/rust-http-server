use std::thread;
use thread_id;


pub struct Debugger {
    pub enabled: bool
}
  
impl Debugger {
  pub fn debug_line(&self, debug_line: &str) {
    if self.enabled {
      println!("[*] {}", debug_line)
  }
}
  
  pub fn debug_request(&self, req: super::super::http::request::HTTPRequest, res: super::super::http::response::HTTPResponse, address: std::net::SocketAddr) {

    if self.enabled {
      println!("[worker ID {}] ({:?}) {:?} {:?} => {:?}!", thread_id::get() % 1000, address, req.verb, req.path, res.status)
    }
  }
}
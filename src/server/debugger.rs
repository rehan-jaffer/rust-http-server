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
      println!("[{}] {:?} {:?} - {:?} {}", "16-09-2021 18:01", address, req.verb, res.status, req.path)
    }
  }
}
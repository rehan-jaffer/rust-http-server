pub struct HTTPRequestParser {
    pub request: Vec<String>
}
  
impl HTTPRequestParser {
    pub fn parse(&mut self) -> Option<super::request::HTTPRequest> {
  
        let parts = self.request[0]
          .split(" ")
          .collect::<Vec<&str>>();
        
        let verb = HTTPRequestParser::get_request_type(parts.clone());
        let path = parts[1].to_string();
        let headers = super::headers::HTTPHeaders::new(self.request[2..].to_vec());
  
        return Some(super::request::HTTPRequest { 
          verb: verb, 
          path: path 
        })
    }
  
    fn get_request_type(request_line_parts: Vec<&str>) -> super::request::HTTPRequestType {
        return match request_line_parts[0] {
          "GET" => super::request::HTTPRequestType::Get,
          "POST" => super::request::HTTPRequestType::Post,
          "UPDATE" => super::request::HTTPRequestType::Update,
          "DELETE" => super::request::HTTPRequestType::Delete,
          _ => super::request::HTTPRequestType::Unknown
      };
    }
  }
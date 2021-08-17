pub struct HTTPHeader {
  key: String,
  value: String
}
  
pub struct HTTPHeaders {
  headers: Vec<HTTPHeader>
}
  
impl HTTPHeaders {
  pub fn new(raw_headers: Vec<String>) -> Self {
  
    let mut header_vec = Vec::new();
    for header in raw_headers {
      let parts = header.split(": ").collect::<Vec<&str>>();
      header_vec.push(HTTPHeader { key: parts[0].to_string(), value: parts[1].to_string() });
      println!("{} -> {}", parts[0], parts[1]);
    }
  
    return Self {
      headers: header_vec
    }
  }
}
  
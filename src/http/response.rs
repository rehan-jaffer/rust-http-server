use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum HTTPResponseType {
    Ok,
    Forbidden,
    InternalError,
    FileNotFound
}

impl HTTPResponse {

}

#[derive(Clone)]
pub struct HTTPResponse {
  pub status: HTTPResponseType,
  pub content: String
}


impl Display for HTTPResponse {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let response_string = match self.status {
            HTTPResponseType::Ok => "200 Ok",
            HTTPResponseType::Forbidden => "401 Forbidden",
            HTTPResponseType::InternalError => "500 Error",
            HTTPResponseType::FileNotFound => "404 Not Found"
          };
        return write!(f, "HTTP/1.1 {}\r\nConnection: Closed\r\n\r\n{}\r\n", response_string, self.content);
    }
}
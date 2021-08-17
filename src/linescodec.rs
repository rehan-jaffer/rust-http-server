use std::io::{self, BufRead, Write};
use std::net::TcpStream;

pub struct LinesCodec {
  reader: io::BufReader<TcpStream>,
  writer: io::LineWriter<TcpStream>
}

impl LinesCodec {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
      let writer = io::LineWriter::new(stream.try_clone()?);
      let reader = io::BufReader::new(stream);
      Ok(Self { reader, writer })
    }

    pub fn send(&mut self, res: super::http::response::HTTPResponse) {
        self.writer.write_all(format!("{}\r\n", res).as_bytes());
    }

    pub fn get_lines(&mut self) -> Result<Vec<String>, &str> {
      let mut lines = vec![];

      'outer: loop {
        let mut message = String::new();
        match self.reader.read_line(&mut message) {
            Ok(2) => {
              if (message == "\r\n") {
                  break 'outer;
              }
            },
            Ok(n) => {
              message.pop();
              lines.push(message);
            },
            Err(e) => {
                break;
            }
          }
    
      }
      return Ok(lines);
    }
}
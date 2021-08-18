use std::fs::{ File as FileRead };
use std::io::prelude::*;
use std::error::Error;

const WEB_ROOT : &str = "./public/";

pub struct File {

}

impl File {

    fn run_transforms(path: &str) -> String {

     let new_path = path.to_string();

     return match path.chars().last() {
        Some('/') => { format!("{}{}", path, "index.html") }
        _ => { new_path }
      };

    }

    fn read_file(path: &str) -> Result<String, ()> {

        let final_path = format!("{}{}", WEB_ROOT, File::run_transforms(path));
        match FileRead::open(final_path) {
          Ok(mut file_contents) => {
            let mut contents = String::new();
            file_contents.read_to_string(&mut contents).unwrap();
            Ok(contents)    
          }
          Err(error) => {
            Err(())
          }
        }
    }

    pub fn execute(request: &super::super::super::http::request::HTTPRequest) -> super::super::super::http::response::HTTPResponse {

      match File::read_file(request.path.as_str()) {
          Ok(content) => {
            return super::super::super::http::response::HTTPResponse { 
                status: super::super::super::http::response::HTTPResponseType::Ok,
                content: content
               };         
          }
          Err(()) => {
            return super::super::super::http::response::HTTPResponse { 
                status: super::super::super::http::response::HTTPResponseType::FileNotFound,
                content: "".to_string()
               };         
          }
      }
    }
}
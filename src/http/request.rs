#[derive(Debug)]
pub enum HTTPRequestType {
    Post,
    Get,
    Update,
    Delete,
    Unknown
}

pub struct HTTPRequest {
    pub verb: HTTPRequestType,
    pub path: String,
    pub headers: super::headers::HTTPHeaders
}
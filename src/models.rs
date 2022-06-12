use crate::constants::*;
use std::str;


#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            GET_METHOD => Some(HttpMethod::Get),
            POST_METHOD => Some(HttpMethod::Post),
            PUT_METHOD => Some(HttpMethod::Put),
            DELETE_METHOD => Some(HttpMethod::Delete),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    Ok,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    One,
    OnePointOne,
    Two,
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Html,
    PlainText,
    Json,
    Xml,
}

impl Into<String> for ContentType {
    fn into(self) -> String {
        match self {
            ContentType::Html => String::from("text/html"),
            ContentType::PlainText => String::from("text/plain"),
            ContentType::Json => String::from("application/json"),
            ContentType::Xml => String::from("application/xml"),
        }
    }
}

impl HttpVersion {
    pub fn from_string(s: &str) -> Option<Self> {
        if s == HTTP_VERSION_1_0 {
            Some(HttpVersion::One)
        } else if s == HTTP_VERSION_1_1 {
            Some(HttpVersion::OnePointOne)
        } else {
            None
        }
    }
}

pub trait ToHeaderString {
    fn to_header_string(&self) -> &str;
}

impl ToHeaderString for HttpStatus {
    fn to_header_string(&self) -> &str {
        match self {
            HttpStatus::Ok => OK_RESPONSE_HEADER,
            HttpStatus::BadRequest => BAD_REQUEST_RESPONSE_HEADER,
            HttpStatus::NotFound => NOT_FOUND_RESPONSE_HEADER,
            HttpStatus::MethodNotAllowed => METHOD_NOT_ALLOWED_RESPONSE_HEADER,
        }
    }
}

use std::collections::HashMap;

use serde::Deserialize;

use crate::{models::{HttpMethod, HttpVersion}, constants::CONTENT_LENGTH_HEADER_KEY};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub http_version: HttpVersion,
    pub route: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl<'a> HttpRequest {
    pub fn content_length(&'a self) -> usize {
        if let Some(content_length_string_value) = self.headers.get(CONTENT_LENGTH_HEADER_KEY) {
            if let Ok(content_length) = content_length_string_value.parse::<usize>() {
                content_length
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn json<T>(&'a self) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'a>,
    {
        let deserialized = serde_json::from_slice::<T>(&self.body)?;

        Ok(deserialized)
    }
}
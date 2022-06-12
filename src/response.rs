use std::{io::{BufWriter, Write}, net::TcpStream};

use serde::Serialize;

use crate::models::{HttpStatus, ToHeaderString};

#[derive(Debug)]
pub struct HttpResponse {
    pub writer: BufWriter<TcpStream>,
    pub status: HttpStatus,
}

impl HttpResponse {
    pub fn new(writer: BufWriter<TcpStream>) -> Self {
        HttpResponse {
            writer,
            status: HttpStatus::Ok,
        }
    }

    pub fn json<T>(&mut self, value: T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
    {
        self.writer
            .write_all(self.status.to_header_string().as_bytes())
            .expect("could not write status");

        let serialized = serde_json::to_vec(&value)?;

        self.writer
            .write_all(&serialized)
            .expect("Could not write json");

        Ok(())
    }
}
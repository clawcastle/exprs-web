use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::{models::{HttpMethod, HttpVersion}, request::HttpRequest};

#[derive(Debug, Clone)]
pub struct HttpRequestParsingError {
    message: String,
}

impl Display for HttpRequestParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

//TODO: Error handling
pub fn parse_request_from_stream<'a>(stream: TcpStream) -> HttpRequest {
    let mut stream_reader = BufReader::new(stream.try_clone().unwrap());

    let mut request = HttpRequest {
        route: String::from(""),
        headers: HashMap::new(),
        method: HttpMethod::Get,
        http_version: HttpVersion::One,
        body: vec![],
    };

    add_essential_information(&mut stream_reader, &mut request);
    add_headers(&mut stream_reader, &mut request);
    add_body(&mut stream_reader, &mut request);

    request
}

fn add_essential_information(stream_reader: &mut BufReader<TcpStream>, request: &mut HttpRequest) {
    let mut first_line_buffer = String::new();
    stream_reader
        .read_line(&mut first_line_buffer)
        .expect("could not read first line of request");

    let line_parts: Vec<&str> = first_line_buffer.split(' ').collect();

    println!("{:?}", line_parts);

    let method = HttpMethod::from_string(line_parts[0]).expect("Could not parse http method");
    let route = line_parts[1];
    let http_version =
        HttpVersion::from_string(line_parts[2].trim()).expect("Could not parse http version.");

    request.route = String::from(route);
    request.method = method;
    request.http_version = http_version;
}

fn add_headers(stream_reader: &mut BufReader<TcpStream>, request: &mut HttpRequest) {
    let mut line_buffer = String::new();

    let mut headers: HashMap<String, String> = HashMap::new();

    let mut bytes_read = stream_reader.read_line(&mut line_buffer);

    while let Ok(n) = bytes_read {
        if n <= 0 {
            break;
        }

        line_buffer.retain(|c| !c.is_whitespace());

        if line_buffer.len() <= 0 {
            break;
        }

        let parts: Vec<&str> = line_buffer.split(':').collect();

        if parts.len() >= 2 {
            headers.insert(parts[0].to_owned().to_lowercase(), parts[1].to_owned());
        }

        line_buffer.clear();
        bytes_read = stream_reader.read_line(&mut line_buffer);
    }

    request.headers = headers;
}

fn add_body(stream_reader: &mut BufReader<TcpStream>, request: &mut HttpRequest) {
    let mut consumed: usize = 0;

    let content_length = request.content_length();
    let buffer_length = stream_reader.buffer().len();

    if buffer_length > 0 {
        if content_length > 0 && content_length <= buffer_length {
            let mut buffer = vec![0u8; content_length];

            stream_reader
                .read_exact(&mut buffer)
                .expect("Failed to read content_length bytes");

            request.body = buffer;
        } else {
            let mut buffer: Vec<u8> = vec![];

            if let Ok(n) = stream_reader.fill_buf() {
                buffer.extend_from_slice(n);
                consumed = n.len();
            }

            stream_reader.consume(consumed);

            request.body = buffer;
        }
    }
}

use crate::{
    models::{
        HttpMethod, HttpStatus, ToHeaderString,
    },
    request_parser::parse_request_from_stream, response::HttpResponse, request::HttpRequest, service_collection::{ServiceCollection, RequestHandlerParams, RequestHandler},
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufWriter, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

pub struct HttpApp {
    host: String,
    services: ServiceCollection,
}

impl HttpApp {
    pub fn new(host: &str, port: u16) -> Self {
        HttpApp {
            host: format!("{host}:{port}"),
            services: ServiceCollection::new(),
        }
    }

    fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), io::Error> {
        let req = parse_request_from_stream(stream.try_clone().unwrap());

        let writer = BufWriter::new(stream.try_clone().unwrap());
        let mut res = HttpResponse::new(writer);

        let static_files_dir = Path::new(&self.services.static_files_dir);
        let route_path = Path::new(&req.route[1..]);

        let static_path = Path::join(static_files_dir, route_path);
        let static_path_str = static_path.to_str().unwrap();

        if let Some(request_handler) = self.services.get_handler(&req, &mut res) {
            match &request_handler.handler {
                RequestHandlerParams::RouteHandler(route_handler) => {
                    route_handler(&req, &mut res);
                }
                RequestHandlerParams::None => {
                    println!("no route matched");
                }
            }
        } else if self.services.static_file_path_exists(static_path_str) {
            self.send_static_file(static_path_str, &mut res);
        } else {
            res.writer
                .write_all(res.status.to_header_string().as_bytes())?;

            res.writer.flush()?
        }

        res.writer.flush()
    }

    fn send_static_file(&self, path: &str, res: &mut HttpResponse) {
        if let Ok(file_content) = fs::read(path) {
            res.writer
                .write_all(HttpStatus::Ok.to_header_string().as_bytes())
                .expect("write status");

            res.writer
                .write_all(&file_content)
                .expect("could not write static file");
        } else {
            res.status = HttpStatus::NotFound;
        }
    }

    pub fn static_file(mut self, path: &str) -> Self {
        self.services.add_static_file(path);

        self
    }

    pub fn get(
        mut self,
        route: &'static str,
        handler_fn: impl Fn(&HttpRequest, &mut HttpResponse) -> () + 'static,
    ) -> Self {
        let handler = RequestHandler {
            method: HttpMethod::Get,
            handler: RequestHandlerParams::RouteHandler(Box::from(handler_fn)),
        };

        self.services.add_service(route, handler);

        self
    }

    pub fn post(
        mut self,
        route: &'static str,
        handler_fn: impl Fn(&HttpRequest, &mut HttpResponse) -> () + 'static,
    ) -> Self {
        let handler = RequestHandler {
            method: HttpMethod::Post,
            handler: RequestHandlerParams::RouteHandler(Box::from(handler_fn)),
        };

        self.services.add_service(route, handler);

        self
    }

    pub fn put(
        mut self,
        route: &'static str,
        handler_fn: impl Fn(&HttpRequest, &mut HttpResponse) -> () + 'static,
    ) -> Self {
        let handler = RequestHandler {
            method: HttpMethod::Put,
            handler: RequestHandlerParams::RouteHandler(Box::from(handler_fn)),
        };

        self.services.add_service(route, handler);

        self
    }

    pub fn delete(
        mut self,
        route: &'static str,
        handler_fn: impl Fn(&HttpRequest, &mut HttpResponse) -> () + 'static,
    ) -> Self {
        let handler = RequestHandler {
            method: HttpMethod::Delete,
            handler: RequestHandlerParams::RouteHandler(Box::from(handler_fn)),
        };

        self.services.add_service(route, handler);

        self
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.host).unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            match self.handle_connection(&mut stream) {
                Ok(_) => {
                    println!("responded")
                }
                Err(err) => {
                    println!("err: {}", err);
                }
            }
        }
    }
}

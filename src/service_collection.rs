use std::collections::{HashMap, HashSet};

use crate::{request::HttpRequest, response::HttpResponse, models::{HttpMethod, HttpStatus}};

pub enum RequestHandlerParams {
    RouteHandler(Box<dyn Fn(&HttpRequest, &mut HttpResponse) -> () + 'static>),
    None,
}

pub struct RequestHandler {
    pub method: HttpMethod,
    pub handler: RequestHandlerParams,
}

pub struct ServiceCollection {
    services: HashMap<String, Vec<RequestHandler>>,
    pub static_file_paths: HashSet<String>,
    pub static_files_dir: String,
}

impl ServiceCollection {
    pub fn new() -> Self {
        ServiceCollection { services: HashMap::new(), static_file_paths: HashSet::new(), static_files_dir: String::from("./") }
    }

    pub fn add_service(&mut self, route: &str, handler: RequestHandler) {
        if !self.services.contains_key(route) {
            self.services.insert(String::from(route), vec![]);
        }

        match self.services.get_mut(route) {
            Some(handlers) => handlers.push(handler),
            None => panic!("Failed adding handler."),
        }
    }

    pub fn add_static_file(&mut self, path: &str) {
        self.static_file_paths.insert(String::from(path));
    }

    pub fn get_handler(&self, req: &HttpRequest, res: &mut HttpResponse) -> Option<&RequestHandler> {
        if let Some(handlers) = self.services.get(&req.route) {
            if let Some(handler) = handlers.iter().find(|h| h.method == req.method) {
                Some(handler)
            } else {
                res.status = HttpStatus::MethodNotAllowed;
                None
            }
        } else {
            res.status = HttpStatus::NotFound;
            None
        }
    }

    pub fn static_file_path_exists(&self, path: &str) -> bool {
        self.static_file_paths.contains(path)
    }
}
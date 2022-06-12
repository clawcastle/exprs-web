pub const GET_METHOD: &'static str = "GET";
pub const POST_METHOD: &'static str = "POST";
pub const PUT_METHOD: &'static str = "PUT";
pub const DELETE_METHOD: &'static str = "DELETE";
pub const OK_RESPONSE_HEADER: &'static str = "HTTP/1.1 200 OK\r\n\r\n";
pub const BAD_REQUEST_RESPONSE_HEADER: &'static str = "HTTP/1.1 400 OK\r\n\r\n";
pub const NOT_FOUND_RESPONSE_HEADER: &'static str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const METHOD_NOT_ALLOWED_RESPONSE_HEADER: &'static str =
    "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";
pub const CONTENT_LENGTH_HEADER_KEY: &'static str = "content-length";

pub const HTTP_VERSION_1_0: &'static str = "HTTP/1.0";
pub const HTTP_VERSION_1_1: &'static str = "HTTP/1.1";

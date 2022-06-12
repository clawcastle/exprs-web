use exprs::{app::*, models::HttpMethod};
use serde::{Deserialize, Serialize};

pub fn test(method: HttpMethod) {
    println!("{:?}", method)
}

fn main() {
    let app = HttpApp::new("127.0.0.1", 5000)
        .get("/", |_req, _res| test(HttpMethod::Get))
        .post("/post", |req, res| {
            println!("{}", req.content_length());

            let user = req.json::<User>();

            println!("{:?}", user);

            let product = Product {
                id: 42,
                name: String::from("product 1"),
            };

            res.json(product).expect("could not write json to response");
        })
        .static_file("./test.html");

    app.start();
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

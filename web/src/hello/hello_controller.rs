use poem::web::Json;
use poem::{get, handler, listener::TcpListener, web::Path, Body, IntoResponse, Route, Server};
use serde::Deserialize;

pub const HELLO_GET_PATH: &str = "/hello/:name";
pub const HELLO_POST_PATH: &str = "/hello";

#[handler]
pub fn hello_get(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[derive(Debug, Deserialize)]
struct Params {
    name: String,
}

#[handler]
pub fn hello_post(req: Json<Params>) -> String {
    format!("hello: {}", req.name)
}

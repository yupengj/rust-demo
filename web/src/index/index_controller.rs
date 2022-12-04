use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

pub const INDEX_GET_PATH_1: &str = "/";
pub const INDEX_GET_PATH_2: &str = "/index";

#[handler]
pub fn index_get() -> String {
    format!("欢迎来到 Rust Poem")
}

mod hello;
mod index;
mod user;

use poem::{
    get, handler, listener::TcpListener, post, web::Path, Body, IntoResponse, Route, Server,
};

use poem_openapi::{
    payload::Json,
    types::{Email, Password},
    ApiResponse, Object, OpenApi, OpenApiService, Tags,
};
use slab::Slab;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api::default(), "Users", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    let listener = TcpListener::bind("127.0.0.1:3000");
    let server = Server::new(listener);
    let route = Route::new();

    let app = route
        .at(hello::HELLO_GET_PATH, get(hello::hello_get))
        .at(hello::HELLO_POST_PATH, post(hello::hello_post))
        .at(index::INDEX_GET_PATH_1, get(index::index_get))
        .at(index::INDEX_GET_PATH_2, get(index::index_get));

    server.run(app).nest("/api", api_service).nest("/", ui).await
}

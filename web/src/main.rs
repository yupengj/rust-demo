mod hello;
mod index;
mod user;

use poem::{
    get, handler, listener::TcpListener, post, web::Path, Body, IntoResponse, Route, Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:3000");
    let server = Server::new(listener);
    let route = Route::new();

    let app = route
        .at(hello::HELLO_GET_PATH, get(hello::hello_get))
        .at(hello::HELLO_POST_PATH, post(hello::hello_post))
        .at(index::INDEX_GET_PATH_1, get(index::index_get))
        .at(index::INDEX_GET_PATH_2, get(index::index_get));

    server.run(app).await
}

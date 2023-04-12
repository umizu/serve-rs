#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("--> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
       .serve(routes_hello.into_make_service())
       .await
       .unwrap();
}

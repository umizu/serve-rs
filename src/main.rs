#![allow(unused)]

use std::net::SocketAddr;

use axum::{handler, response::{Html, IntoResponse}, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("--> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

    Html("hello world")
}

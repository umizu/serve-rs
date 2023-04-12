#![allow(unused)]

use std::net::SocketAddr;

use axum::{handler, response::{Html, IntoResponse}, routing::get, Router, extract::Query};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/books", get(handler_books));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    println!("--> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_books(Query(params): Query<GetBooksParams>) -> impl IntoResponse {
    println!("->> {:12} - books_hanler - {params:?}", "HANDLER");

    let title = params.title.as_deref().unwrap_or("empty");
    Html(format!("books with title: {title}"))
}

#[derive(Debug, Deserialize)]
struct GetBooksParams {
    title: Option<String>,
}

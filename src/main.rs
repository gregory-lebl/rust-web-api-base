#![allow(unused)]

pub use self::error::{Error,Result};

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct HelloParams {
    name: Option<String>,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

fn routes_hello() -> Router {
    return Router::new()
    .route("/hello", get(handler_hello))

}

async fn handler_hello(Query(params): Query<HelloParams>) -> Json<HelloResponse> {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.unwrap_or("World".to_string());

    let response = HelloResponse {
        message: String::from("Hello World in JSON")
    };

    Json(response)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

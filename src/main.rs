#![allow(unused)]

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::{Connection, PgConnection};
use std::error::Error;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
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

async fn database_connection() -> Result<(i32), Box<dyn Error>> {
    let url: &str = "postgres://postgres:root@database:5432/rustapi";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    let sum: i32 = res.get("sum");

    println!("1 + 1 = {}", sum);

    Ok(sum);
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
    return Router::new().route("/hello", get(handler_hello));
}

async fn handler_hello(Query(params): Query<HelloParams>) -> Json<HelloResponse> {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    database_connection();

    let name = params.name.unwrap_or("World".to_string());

    let response = HelloResponse {
        message: String::from("Hello World in JSON"),
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

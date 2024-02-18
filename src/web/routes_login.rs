use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error, Result};


pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>>{
  println!("->> {:<12} - api_login", "HANDLER");

  // TODO: Implement real db/auth logic

  if payload.username != "demo1" || payload.password != "welcome" {
    return Err(Error::LoginFail)
  }

  // TODO: Set cookies

  // FIXME: Create a real token
  cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

  // Create the succes body
  let body = Json(json!({
    "result": {
      "sucess": true
    }
  }));

  Ok(body)
}
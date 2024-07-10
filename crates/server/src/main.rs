use axum::{
  response::Html,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};

async fn root() -> Html<&'static str> {
  Html(include_str!("../index.html"))
}

#[derive(Serialize, Deserialize)]
struct Chat {
  messages: Vec<String>,
}

async fn chat(Json(mut chat): Json<Chat>) -> Json<Chat> {
  chat
    .messages
    .push("And how does that make you feel?".to_string());
  Json(chat)
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(root))
    .route("/chat", post(chat));
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
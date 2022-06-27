mod ds;
mod idx;
mod utils;
mod config;
mod wondkv;
mod storage;
mod db_hash;
mod wondkv_test;

use axum::{
    routing::post,
    Router,
    Json,
    Extension,
};
use std::net::SocketAddr;
use serde_json::{Value, json};

use tokio::sync::mpsc;
use tokio::sync::oneshot;

struct Message {
    method: u8,
    key: Option<String>,
    value: Option<String>,
    channel: Option<oneshot::Sender<String>>,
}

impl Message {
    fn new() -> Message {
        Message {
            method: 0,
            key: None,
            value: None,
            channel: None,
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let mut message = Message::new();
    message.method = 0;
    let _ = tx.send(message).await;

    tokio::spawn(async move {
        let app = Router::new()
        .route("/key/get", post(kv_get))
        .route("/key/set", post(kv_set))
        .route("/key/test", post(kv_test))
        .layer(Extension(tx));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3010));
        println!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let config = config::default_config();
    let ret = config.open();
    if ret.is_none() {
        panic!();
    }
    let mut db = ret.unwrap();

    while let Some(message) = rx.recv().await {
        match message.method {
            1 => {
                if let Some(value) = db.hget(message.key.unwrap().into_bytes(), vec![1, 2, 3]) {
                    let _ = message.channel.unwrap().send(std::str::from_utf8(&value).unwrap().to_string());
                } else {
                    let _ = message.channel.unwrap().send("".to_string());
                }
            }
            2 => {
                db.hset(message.key.unwrap().into_bytes(), vec![1, 2, 3], message.value.unwrap().into_bytes());
            }
            _ => ()
        }
    }
}

async fn kv_get(
    Json(payload): Json<serde_json::Value>,
    Extension(state): Extension<mpsc::Sender<Message>>,
) -> Json<Value>  {
    let key = payload.as_object().unwrap().get("key").unwrap().as_str().unwrap().to_string();
    let (tx, rx) = oneshot::channel();
    let mut message = Message::new();
    message.method = 1;
    message.key = Some(key);
    message.channel = Some(tx);
    let _ = state.send(message).await;
    let value: String = rx.await.unwrap();
    if value == "" {
        Json(json!({ "status": 1, "data": value }))
    } else {
        Json(json!({ "status": 0, "data": value }))
    }
}

async fn kv_set(
    Json(payload): Json<serde_json::Value>,
    Extension(state): Extension<mpsc::Sender<Message>>,
) {
    let key = payload.as_object().unwrap().get("key").unwrap().as_str().unwrap().to_string();
    let value = payload.as_object().unwrap().get("value").unwrap().as_str().unwrap().to_string();
    let mut message = Message::new();
    message.method = 2;
    message.key = Some(key);
    message.value = Some(value);
    let _ = state.send(message).await;
}

async fn kv_test() {

}
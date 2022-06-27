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
};
use std::env;
use std::net::SocketAddr;
use std::fs::{self, File};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let current_path = env::current_dir().ok().unwrap();
    let temp_dir = current_path.join("tmp_wondkv_test");
    fs::create_dir(temp_dir.clone()).ok();
    File::create(temp_dir.join("1.data.hash")).ok();

    let app = Router::new()
        .route("/key/get", post(kv_get))
        .route("/key/set", post(kv_set));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3010));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn kv_get(
    Json(payload): Json<serde_json::Value>,
) -> Json<Value> {
    let key = payload.as_object().unwrap().get("key").unwrap().as_str().unwrap().to_string();
    let mut config = config::default_config();
    let current_path = env::current_dir().ok().unwrap();
    let temp_dir = current_path.join("tmp_wondkv_test");
    config.dir_path = temp_dir.to_str().unwrap().to_string();
    let ret = config.open();
    if ret.is_none() {
        panic!();
    }
    let mut db = ret.unwrap();
    if let Some(value) = db.hget(key.into_bytes(), vec![1, 2, 3]) {
        Json(json!({ "status": 0, "data": std::str::from_utf8(&value).unwrap() }))
    } else {
        Json(json!({ "status": 1, "data": "" }))
    }
}

async fn kv_set(
    Json(payload): Json<serde_json::Value>,
) {
    let key = payload.as_object().unwrap().get("key").unwrap().as_str().unwrap().to_string();
    let value = payload.as_object().unwrap().get("value").unwrap().as_str().unwrap().to_string();
    let mut config = config::default_config();
    let current_path = env::current_dir().ok().unwrap();
    let temp_dir = current_path.join("tmp_wondkv_test");
    config.dir_path = temp_dir.to_str().unwrap().to_string();
    let ret = config.open();
    if ret.is_none() {
        panic!();
    }
    let mut db = ret.unwrap();
    db.hset(key.into_bytes(), vec![1, 2, 3], value.into_bytes());
}
mod model;
mod post_repo;

use axum::extract::{Extension, Path};
use axum::handler::{get, post};
use axum::{Json,Router,AddExtensionLayer};
use r2d2::Pool;
use redis::Client;
use serde_json::{json, Value};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let client = Client::open("redis://127.0.0.1").unwrap();
    let pool = Pool::builder().build(client).unwrap();
    let shared_pool = Arc::new(pool);

    let app = Router::new().route("/post", post(post_post)).route("/post/:id", get(get_post).delete(delete_post)).layer(AddExtensionLayer::new(shared_pool));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_post(Path(id_str):Path<String>, repo: Extension<Arc<post_repo::PostRepository>>) -> Json<Value>{
    let repo: Arc<post_repo::PostRepository> = repo.0;
    let id = id_str.parse::<i64>().unwrap();
    let post = repo.get(id).unwrap();
    Json(json!(post))
}

async fn post_post(Path(id_str):Path<String>, payload:Json<model::Post>,  repo: Extension<Arc<post_repo::PostRepository>>){
    let repo: Arc<post_repo::PostRepository> = repo.0;
    let id = id_str.parse::<i64>().unwrap();
    let post: model::Post = payload.0;
    repo.create(id, post);
}

async fn delete_post(Path(id_str):Path<String>, repo: Extension<Arc<post_repo::PostRepository>>){
    let repo: Arc<post_repo::PostRepository> = repo.0;
    let id = id_str.parse::<i64>().unwrap();
    repo.delete(id);
}
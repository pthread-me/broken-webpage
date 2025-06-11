use webserver::*;
use std::path::PathBuf;
use database::db::DbPool;

use tower_http::services::ServeDir;
use axum::{
  response::Redirect, 
  routing::{get, post}, 
  Router, 
  extract::{OriginalUri},
  http
};


#[tokio::main]
async fn main() {
  
  let static_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "static"].iter().collect(); 
  
  let pool = DbPool::new().await;
  pool.unwrap().w_pool;

  let router =  Router::new()
    .route("/", get(|| async {Redirect::permanent("/home")}))
    .route("/home", get(|uri: http::Uri| {index_controller::get_index(OriginalUri(uri)) }))
    .route("/home/{test}", get(|uri: http::Uri| {index_controller::get_index(OriginalUri(uri)) }))
    .route("/home", post(index_controller::arena_handler)) 
    .nest_service("/static", ServeDir::new(static_path));


  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, router).await.unwrap();
}



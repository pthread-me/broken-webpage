use webserver::{index_controller::get_index, *};
use std::path::PathBuf;
use std::sync::Arc;
use database::db::DbPool;

use tower_http::services::ServeDir;
use axum::{
  response::Redirect, 
  routing::{get, post}, 
  Router, 
};


#[tokio::main]
async fn main() {
  
  let static_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "static"].iter().collect(); 
  
  let pool = if let Ok(pool) = DbPool::new().await{
    Arc::new(pool)
  }else{
    panic!("Cant connect to Database")
  };

  
  let router =  Router::new()
    .route("/", get(|| async {
      Redirect::permanent("/home")
      }))
    .route("/home", get(get_index))
    .route("/home", post(index_controller::arena_handler)) 
    .nest_service("/static", ServeDir::new(static_path))
    .with_state(pool.clone());

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, router).await.unwrap();
}



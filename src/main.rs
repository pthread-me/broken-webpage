use broken_webpage::*;

use tower_http::services::ServeDir;

use axum::{
  response::Redirect, 
  routing::get, 
  Router, 
  extract::OriginalUri,
  http
};



// OriginalUri is a wrapper over http::uri that adds more functionalities (aka get
// entire uri) I first extract the http::uri via the closure, then cast and pass to 
// the templating func.
#[tokio::main]
async fn main() {
  let router =  Router::new()
    .route("/", get(|| async {Redirect::permanent("/home")}))
    .route("/home", get(|uri: http::Uri| {
      index_controller::get_index(OriginalUri(uri)) 
    })) 
    .nest_service("/static", ServeDir::new("./static"));


  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, router).await.unwrap();
}



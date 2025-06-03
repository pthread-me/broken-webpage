mod index_template;

use askama::Template;
use index_template::{Page};
use axum::{
  http::StatusCode, routing::get, Router
};
use axum::response::{
  Html, IntoResponse
};

use crate::index_template::MainNavBar;


#[tokio::main]
async fn main() {
  let router = Router::new().route("/", get(get_index));
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, router).await.unwrap();
}

async fn get_index() -> impl IntoResponse {
 
  let mut pages:Vec<Page> = Vec::new();
  pages.push(Page{
    name: "Home",
    link: "#"
  });

  let m_nav_bar = MainNavBar{
    pages,
  };
 
  let reply = m_nav_bar.render().unwrap();
  (StatusCode::OK, Html(reply).into_response())
}

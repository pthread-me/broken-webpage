mod index_template;


use crate::index_template::MainNavBar;
use tower_http::services::ServeDir;
use askama::Template;
use index_template::{Page};

use axum::{
  http::StatusCode, routing::get, Router
};
use axum::response::{
  Html, IntoResponse
};



#[tokio::main]
async fn main() {
  let router =  routes(); 
  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
  axum::serve(listener, router).await.unwrap();
}

fn routes() -> Router{
  Router::new()
    .route("/", get(get_index))
    .nest_service("/static", ServeDir::new("./static"))

}

async fn get_index() -> impl IntoResponse { 
  let mut pages:Vec<Page> = Vec::new();
  pages.push(Page{name: "Home",link: "https://google.com"});
  pages.push(Page{name: "Projects",link: "#"});
  pages.push(Page{name: "Home",link: "#"});
  pages.push(Page{name: "Home",link: "#"});

  let m_nav_bar = MainNavBar{pages};
 
  let reply = m_nav_bar.render().unwrap();
  (StatusCode::OK, Html(reply).into_response())
}

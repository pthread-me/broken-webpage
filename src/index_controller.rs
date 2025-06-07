use crate::nav_controller::NavBar;
use askama::Template;
use axum::{
  http::{StatusCode},
  response::{Html, IntoResponse},
  extract::{OriginalUri}
};


#[derive(Template)]
#[template(path="./index.html")]
struct IndexPage{
  nav: NavBar,
}
pub async fn get_index(uri: OriginalUri)-> impl IntoResponse { 
  let mut path = String::new();
  uri.path().clone_into(&mut path);
  let page = IndexPage {nav: NavBar::new(uri.path())};
  
  let reply = page.render().unwrap_or(String::from("Could not render page :(")); 
  (StatusCode::OK, Html(reply).into_response())
}




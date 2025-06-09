use crate::nav_controller::NavBar;
use askama::Template;
use axum::{
  extract::{OriginalUri, Form}, 
  http::StatusCode, 
  response::{Html, IntoResponse}
};

use serde::Deserialize;
#[derive(Template)]
#[template(path="./index.html")]
struct IndexPage{
  nav: NavBar,
}

pub async fn get_index(uri: OriginalUri)-> impl IntoResponse { 
  let page = IndexPage {nav: NavBar::new(uri.path())};
  
  let reply = page.render().unwrap_or(String::from("Could not render page :(")); 
  (StatusCode::OK, Html(reply).into_response())
}


#[derive(Deserialize)]
pub struct ArenaPost{
  content: String,
}

pub async fn arena_handler(Form(param): Form<ArenaPost>) -> impl IntoResponse{
  let reply = match param.content.as_str() {
    "about me" => "test".to_owned(),
    _ => "fuck off".to_owned()
  };

  (StatusCode::OK, Html(reply).into_response())
}

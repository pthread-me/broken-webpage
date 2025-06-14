use crate::nav_controller::NavBar;
use database::db::DbPool;
use askama::Template;
use std::sync::Arc;
use serde::Deserialize;
use axum::{
  extract::{OriginalUri, Form, State}, 
  http::StatusCode, 
  response::{Html, IntoResponse},
  debug_handler
};



#[derive(Template)]
#[template(path="./index.html")]
struct IndexPage{
  nav: NavBar,
}

#[derive(Deserialize)]
pub struct PostData{
  pub content: String,
}


#[debug_handler]
pub async fn get_index(uri: OriginalUri)-> impl IntoResponse { 
  let page = IndexPage {nav: NavBar::new(uri.path())};
  
  let reply = page.render().unwrap_or(String::from("Could not render page :(")); 
  (StatusCode::OK, Html(reply).into_response())
}


/*
 * Note to self, Form() always consumes everything, so must be the last param
 */
#[debug_handler]
pub async fn arena_handler(State(db_pool): State<Arc<DbPool>>, Form(form_data): Form<PostData>) -> impl IntoResponse{
  let reply:String = db_pool.as_ref()
    .get_entry(&form_data.content).await
    .unwrap_or_else(|_| "Empty Arena".to_string());

  (StatusCode::OK, Html(reply).into_response())
}

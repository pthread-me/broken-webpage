use sqlx::{
  Sqlite,
   SqlitePool,
};
use sqlx::sqlite::{
  SqliteConnectOptions, 
  SqliteJournalMode::Wal, 
  SqlitePoolOptions
};
use std::{
  str::FromStr,
  path::PathBuf
};

use anyhow::{Context, Result};
use crate::cmark_wrapper::to_html;
use sqlx::prelude::FromRow;



/*
 *  Note to self, SqlitePool is type alias to Pool
*/
#[derive(Debug, Clone)]
pub struct DbPool{
  pub w_pool: SqlitePool,
  pub r_pool: SqlitePool,
}

#[derive(FromRow)]
struct DbRow{
  _id: i64,
  _title: String,
  _summary: Option<String>,
  html: Option<String>,
  markdown: String,
}

impl DbPool{
    pub async fn new() -> Result<DbPool>{
      let db_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "webserver.db"].iter().collect(); 
      
      // Joural mode = Wal is not necessary for 1 writer but i might scale in the future
      let pool_opt = SqliteConnectOptions::from_str(
          &db_path.to_str().map(|path| format!("{path}?mode=rw")).unwrap()
        )?
        .journal_mode(Wal);

      // Initializing for read and write pools
      let r_pool = SqlitePoolOptions::new()
        .min_connections(4)
        .max_connections(8)
        .connect_with(pool_opt.clone().read_only(true)).await
        .map_err(|err| {
          eprintln!("Could not open read only connections, in DbPool");
          err
        }).unwrap();
      let w_pool = SqlitePoolOptions::new()
        .min_connections(1)
        .max_connections(1)
        .connect_with(pool_opt.clone().read_only(false)).await
        .map_err(|err| {
          eprintln!("Could not open read-write connection in DbPool");
          err
        }).unwrap();

      Ok(DbPool{w_pool, r_pool,})
    }


    // First attempt to read entry, if not rendered, switch to writing mode
    // render then update db and return the expected html
    // all errs are propagated to the caller except when update
    // changes undesired rows in db, in that case idk what to do so panic
    pub async fn get_entry(&self, title: &str) -> Result<String>{
      let reader = &self.r_pool;
      let entry: DbRow = sqlx::query_as("SELECT * from Blogs where title = $1")
        .bind(title)
        .fetch_one(reader).await?;

      if let Some(html) = entry.html{
        return Ok(html)
      }

      let html: String = to_html(&mut entry.markdown.clone());
      
      let writer = &self.w_pool;
      let update_res = sqlx::query::<Sqlite>("UPDATE BLOGS SET html = $1 WHERE title = $2")
        .bind(html)
        .bind(title)
        .execute(writer).await?;

      assert!(update_res.rows_affected() == 1, "Update operation corrupted DB");

      let entry:DbRow = sqlx::query_as("SELECT * FROM Blogs WHERE title = $1")
        .bind(title)
        .fetch_one(writer).await?;


      entry.html.context("Rendering issue probably")
  }
}


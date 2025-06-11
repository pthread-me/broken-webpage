use sqlx::sqlite::{
  SqliteConnectOptions, SqliteJournalMode::Wal, SqlitePoolOptions
};
use std::{
  str::FromStr,
  path::PathBuf
};


use sqlx::SqlitePool;
use anyhow::Result;

pub struct DbPool{
  pub w_pool: SqlitePool,
  pub r_pool: SqlitePool,
}

impl DbPool{
  pub async fn new() -> Result<DbPool>{
    let db_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "webserver.db"].iter().collect(); 
     

    let pool_opt = SqliteConnectOptions::from_str(
        &db_path.to_str().map(|path| format!("{path}?mode=rw")).unwrap()
      )?
      .journal_mode(Wal);

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

   Ok(DbPool{
      w_pool,
      r_pool,
    })
  }
}


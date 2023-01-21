use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Connection};
use std::fs;
use std::time::Duration;

// const PG_HOST: &str = "157.245.96.119:5432";
const PG_HOST: &str = "206.189.140.208:5432";

// app db
const PG_APP_DB: &str = "soumyadeepmukherjee";
const PG_APP_USER: &str = "soumyadeepmukherjee";
const PG_APP_PWD: &str = "ILovePostgres";
const PG_APP_MAX_CON: u32 = 5;
// sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
	new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, PG_APP_MAX_CON).await
}

async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
	let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
	PgPoolOptions::new()
		.max_connections(max_con)
		.acquire_timeout(Duration::from_millis(500)) // Needs to find replacement
		.connect(&con_string)
		.await
}

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
// endregion: Test
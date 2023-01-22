use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Connection};
use std::fs;
use std::time::Duration;
use dotenv::dotenv;

const PG_APP_MAX_CON: u32 = 5;

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
	dotenv().ok();
	
	for (key, value) in std::env::vars() {
		println!("{key}: {value}");
	}
	
	let host = std::env::var("HOST").unwrap_or("localhost".to_string());
	let db_name = std::env::var("DB_NAME").unwrap_or("viz".to_string());
	let db_user = std::env::var("DB_USER").unwrap_or("viz".to_string());
	let db_pass = std::env::var("DB_PASS").unwrap_or("viz".to_string());

	println!("host: {}", host);
	println!("db_name: {}", db_name);
	println!("db_user: {}", db_user);
	println!("db_pass: {}", db_pass);

	new_db_pool(host, db_name, db_user, db_pass, PG_APP_MAX_CON).await
}

async fn new_db_pool(host: std::string::String, db: std::string::String, user: std::string::String, pwd: std::string::String, max_con: u32) -> Result<Db, sqlx::Error> {
	let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
	println!("con_string: {}", con_string);
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
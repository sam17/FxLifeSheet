use model::init_db;
use std::env;
use std::sync::Arc;
use web::start_web;
mod model;
mod web;

const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
	// compute the web_folder
	if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=logger_name1=debug,logger_name2=info` to see debug logs, this only shows access logs.
        env::set_var("RUST_LOG", "access");
    }
	pretty_env_logger::init();

	let web_port = DEFAULT_WEB_PORT;

	// get the database
	// TODO - loop until valid DB
	let db = init_db().await.expect("Cannot init db");

	let db = Arc::new(db);

	// start the server
	match start_web(web_port, db).await {
		Ok(_) => println!("Server ended"),
		Err(ex) => println!("ERROR - web server failed to start. Cause {:?}", ex),
	}
}
extern crate models;
mod api;
mod utils;
mod daos;

use std::env;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use crate::api::raw_data::raw_data_rest_filters;
use crate::api::viz_categories::viz_categories_rest_filters;
use crate::api::viz_metadata::viz_metadata_rest_filters;
use crate::api::viz_questions::viz_questions_rest_filters;
use crate::utils::db::{Db, init_db};
use crate::utils::error::{Error, WebErrorMessage};

async fn start_web(web_port: u16, db: Arc<Db>) -> Result<(), Error> {
    // Apis
    let raw_data_apis = raw_data_rest_filters("api", &db);
    let metadata_apis = viz_metadata_rest_filters("api", &db);
    let questions_apis = viz_questions_rest_filters("api", &db);
    let categories_apis = viz_categories_rest_filters("api", &db);

    // Static content
    let static_s = warp::fs::dir("../frontend/build/");
    // Combine all routes

    let cors = warp::cors().allow_any_origin();
    let log = warp::log("access");

    // Combine all routes
    let routes = raw_data_apis.or(metadata_apis).or(questions_apis).or(categories_apis)
        .or(static_s).recover(handle_rejection).with(cors).with(log);

    println!("Start 0.0.0.0:{}", web_port);
    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    // Print to server side
    println!("ERROR - {:?}", err);

    // TODO - Call log API for capture and store

    // Build user message
    let user_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.typ.to_string(),
        None => "Unknown".to_string(),
    };

    let result = json!({ "errorMessage": user_message });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(result, warp::http::StatusCode::BAD_REQUEST))
}


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


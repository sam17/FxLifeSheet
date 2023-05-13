extern crate models;
mod api;
mod utils;
mod daos;

use std::borrow::Cow;
use std::env;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;
use crate::api::commands::commands_rest_filters;
use crate::api::raw_data::raw_data_rest_filters;
use crate::api::viz_categories::viz_categories_rest_filters;
use crate::api::viz_metadata::viz_metadata_rest_filters;
use crate::api::viz_questions::viz_questions_rest_filters;
use crate::utils::db::{Db, init_db};
use crate::utils::error::{Error, ModelError, WebErrorMessage};

async fn start_web(web_port: u16, db: Arc<Db>) -> Result<(), Error> {

    // Apis
    let raw_data_apis = raw_data_rest_filters("api", &db);
    let metadata_apis = viz_metadata_rest_filters("api", &db);
    let questions_apis = viz_questions_rest_filters("api", &db);
    let categories_apis = viz_categories_rest_filters("api", &db);
    let commands_apis = commands_rest_filters("api", &db);

    // Static content
    let static_s = warp::fs::dir("../frontend/build/");
    // Combine all routes

    let cors = warp::cors().allow_any_origin();
    let log = warp::log("access");

    // Combine all routes
    let routes = raw_data_apis.or(commands_apis).or(metadata_apis).or(questions_apis).or(categories_apis)
        .or(static_s).recover(handle_rejection).with(cors).with(log);

    println!("Start 0.0.0.0:{}", web_port);
    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: Cow<str>;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = Cow::Borrowed("Not Found");
    } else if let Some(model_error) = err.find::<ModelError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        let detailed_message = format!("ModelError at {}: {}: {}", file!(), line!(), model_error);
        message = Cow::Owned(detailed_message);
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = Cow::Borrowed("Method Not Allowed");
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = Cow::Borrowed("Internal Server Error");
    }

    let json = warp::reply::json(&json!({
        "status": code.as_u16(),
        "message": message,
    }));

    Ok(warp::reply::with_status(json, code))
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


use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;
use crate::daos::bot_commands_dao::BotCommands;
use crate::utils::db::Db;
use crate::utils::filter_utils;

pub fn commands_rest_filters(
    base_path: &'static str,
    db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let data_path = warp::path(base_path).and(warp::path("commands"));
    let common = filter_utils::with_db(db.clone());

    let list = data_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(get_all_commands);

    let get = data_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(get_commands_by_name);

    get.or(list)
}

async fn get_all_commands(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let commands = BotCommands::list(&db).await?;
    let response = json!(commands);
    Ok(warp::reply::json(&response))
}


async fn get_commands_by_name(db: Arc<Db>, name: String) -> Result<Json, warp::Rejection> {
    let commands = BotCommands::get_by_name(&db, name).await?;
    let response = json!(commands);
    Ok(warp::reply::json(&response))
}
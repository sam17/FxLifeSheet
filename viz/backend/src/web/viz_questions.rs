use crate::model::{Db, VizQuestions};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn viz_questions_rest_filters(
    base_path: &'static str,
    db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let data_path = warp::path(base_path).and(warp::path("questions"));
    // let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
    let common = super::filter_utils::with_db(db.clone());

    let get = data_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(questions_list_is_visible);
    get
}

async fn questions_list_is_visible(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let questions = VizQuestions::get_visible_list(&db).await?;
    let response = json!(questions);
    Ok(warp::reply::json(&response))
    // json_response(questions)
}

use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;
use models::models::questions::viz_questions::VizQuestionsQuery;
use crate::daos::viz_questions_dao::VizQuestions;
use crate::utils::db::Db;
use crate::utils::filter_utils;

pub fn viz_questions_rest_filters(
    base_path: &'static str,
    db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let data_path = warp::path(base_path).and(warp::path("questions"));
    let common = filter_utils::with_db(db.clone());

    // get with query params `GET questions/?category=foo&is_visible=true`
    let get_with_query = data_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and(warp::query::<VizQuestionsQuery>())
        .and_then(questions_with_query);

    get_with_query
}

async fn questions_with_query(db: Arc<Db>, query: VizQuestionsQuery) -> Result<Json, warp::Rejection> {
    let is_visible = query.is_visible.unwrap_or(false);
    let category_name = query.category;

    let questions = VizQuestions::get_questions_with_query(&db, category_name, is_visible).await?;
    let response = json!(questions);
    Ok(warp::reply::json(&response))
}


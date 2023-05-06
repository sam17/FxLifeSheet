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

    let get_child_questions = data_path
        .and(common.clone())
        .and(warp::path::param::<String>())
        .and(warp::path("child"))
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and_then(child_questions);

    get_with_query.or(get_child_questions)
}

async fn questions_with_query(db: Arc<Db>, query: VizQuestionsQuery) -> Result<Json, warp::Rejection> {
    let is_visible = query.is_visible.unwrap_or(false);
    let category_name = query.category;
    let command = query.command;

    let questions = VizQuestions::get_questions_with_query(&db,
                                                           category_name,
                                                           is_visible, command).await?;
    let response = json!(questions);
    Ok(warp::reply::json(&response))
}

async fn child_questions(db: Arc<Db>, parent_key: String, answer_option: String) -> Result<Json, warp::Rejection> {
    let questions
        = VizQuestions::get_child_questions(&db, &parent_key, &answer_option).await?;
    let response = json!(questions);
    Ok(warp::reply::json(&response))
}


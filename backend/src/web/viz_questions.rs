use crate::model::{Db, VizQuestions};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct VizQuestionsQuery {
    category: Option<String>,
    is_visible: Option<bool>,
}

pub fn viz_questions_rest_filters(
    base_path: &'static str,
    db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let data_path = warp::path(base_path).and(warp::path("questions"));
    // let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
    let common = super::filter_utils::with_db(db.clone());

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
    let is_visible = query.is_visible;
    let category = query.category;

    let unwrapped_visibility = match is_visible {
        Some(b) => b,
        None => false,
    };

    let unwrapped_category = match category {
        Some(s) => s,
        None => "".to_string(),
    };

    let questions = VizQuestions::get_questions_with_query(&db, unwrapped_category, unwrapped_visibility).await?;
    let response = json!(questions);
    Ok(warp::reply::json(&response))
}

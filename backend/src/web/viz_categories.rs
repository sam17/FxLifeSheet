use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;
use crate::models::core::db::Db;
use crate::models::questions::viz_categories_dao::VizCategories;

pub fn viz_categories_rest_filters(
    base_path: &'static str,
    db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let data_path = warp::path(base_path).and(warp::path("categories"));
    let common = super::filter_utils::with_db(db.clone());

    let get = data_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(get_all_categories);
    get
}

async fn get_all_categories(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    println!("get_all_categories");
    let categories = VizCategories::get_all_categories(&db).await?;
    let response = json!(categories);
    Ok(warp::reply::json(&response))
}

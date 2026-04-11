use crate::model::{Db, RawData};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn raw_data_rest_filters(
	base_path: &'static str,
	db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let data_path = warp::path(base_path).and(warp::path("data"));
	// let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
	let common = super::filter_utils::with_db(db.clone());

	data_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(data_get_by_key)
}

async fn data_get_by_key(db: Arc<Db>, key: String) -> Result<Json, warp::Rejection> {
	let data = RawData::get_by_key(&db,  key).await?;
	json_response(data)
}

// region:    Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
	let response = json!({ "data": data });
	Ok(warp::reply::json(&response))
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
// endregion: Test

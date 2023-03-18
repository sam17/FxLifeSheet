use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;
use crate::models::collector::raw_data_dao::RawData;
use crate::models::core::db::Db;

pub fn raw_data_rest_filters(
	base_path: &'static str,
	db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let data_path = warp::path(base_path).and(warp::path("collector"));
	// let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
	let common = super::filter_utils::with_db(db.clone());

	// LIST raw_data `GET collector/`
	let list = data_path
		.and(warp::get())
		.and(warp::path::end())
		.and(common.clone())
		.and_then(data_get_all);

	// // GET todo `GET /todos/100`
	// let get = data_path
	// 	.and(warp::get())
	// 	.and(common.clone())
	// 	.and(warp::path::param())
	// 	.and_then(data_get);

	let get = data_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(data_get_by_key);

	list.or(get)
}

async fn data_get_all(db: Arc<Db>) -> Result<Json, warp::Rejection> {
	let raw_data = RawData::list(&db).await?;
	json_response(raw_data)
}

async fn data_get_by_key(db: Arc<Db>, key: String) -> Result<Json, warp::Rejection> {
	let data = RawData::get_by_key(&db,  key).await?;
	json_response(data)
}

// region:    Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
	let response = json!({ "collector": data });
	Ok(warp::reply::json(&response))
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
// endregion: Test

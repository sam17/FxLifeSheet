use crate::model::{Db, RawData};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn todo_rest_filters(
	base_path: &'static str,
	db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let data_path = warp::path(base_path).and(warp::path("data"));
	// let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
	let common = super::filter_utils::with_db(db.clone());

	// LIST todos `GET todos/`
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

	// GET todo `GET /data/happyLevels`
	let get = data_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(data_get_by_key);

	list.or(get)
}

async fn data_get_all(db: Arc<Db>) -> Result<Json, warp::Rejection> {
	// json_response(json!({ "id": "test" }))
	println!("DB: {:?}", db);
	let raw_data = RawData::list(&db).await?;
	json_response(raw_data)
}

async fn data_get_by_key(db: Arc<Db>, key: String) -> Result<Json, warp::Rejection> {
	let data = RawData::getByKey(&db,  key).await?;
	json_response(data)
}

// async fn data_get(db: Arc<Db>, id: i64) -> Result<Json, warp::Rejection> {
// 	let todo = RawData::getAll(&db, id).await?;
// 	json_response(todo)
// }

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

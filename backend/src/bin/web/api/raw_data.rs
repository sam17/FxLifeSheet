use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;
use models::models::collector::raw_data::PublishableDataObj;
use crate::daos::raw_data_dao::RawData;
use crate::utils::db::Db;
use crate::utils::filter_utils;

extern crate models;


pub fn raw_data_rest_filters(
	base_path: &'static str,
	db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let data_path = warp::path(base_path).and(warp::path("data"));
	// let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
	let common = filter_utils::with_db(db.clone());

	let list = data_path
		.and(warp::get())
		.and(warp::path::end())
		.and(common.clone())
		.and_then(data_get_all);

	let get = data_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(data_get_by_key);

	let post = data_path
		.and(warp::post())
		.and(common.clone())
		.and(warp::body::json())
		.and_then(data_post);

	list.or(get).or(post)
}


// function to post data to the database in PublishableDataObj format
pub async fn data_post(db: Arc<Db>, data: PublishableDataObj) -> Result<Json, warp::Rejection> {
	let data = RawData::insert_data(&db, &data).await?;
	json_response(data)
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
	let response = json!({ "data": data });
	Ok(warp::reply::json(&response))
}
// endregion: Utils



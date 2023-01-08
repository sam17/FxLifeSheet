use crate::model::{Db, TodoMac, TodoPatch};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn todo_rest_filters(
	base_path: &'static str,
	db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let todos_path = warp::path(base_path).and(warp::path("todos"));
	// let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
	let common = super::filter_utils::with_db(db.clone());

	// LIST todos `GET todos/`
	let list = todos_path
		.and(warp::get())
		.and(warp::path::end())
		.and(common.clone())
		.and_then(todo_list);

	// GET todo `GET /todos/100`
	let get = todos_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(todo_get);
	
	list.or(get)
}

async fn todo_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
	// json_response(json!({ "id": "test" }))
	println!("DB: {:?}", db);
	let todos = TodoMac::list(&db).await?;
	json_response(todos)
}

async fn todo_get(db: Arc<Db>, id: i64) -> Result<Json, warp::Rejection> {
	let todo = TodoMac::get(&db, id).await?;
	json_response(todo)
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

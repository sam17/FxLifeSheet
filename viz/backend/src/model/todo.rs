use super::db::Db;
use crate::model;
use crate::security::UserCtx;
use serde::{Deserialize, Serialize};
use sqlb::{HasFields, Raw};

// region:    Todo Types
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
	pub id: i32,
	pub timestamp: i64,
	pub value: String,
}

#[derive(sqlb::Fields, Default, Debug, Clone, Deserialize)]
pub struct TodoPatch {
	pub title: Option<String>,
	pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
	Open,
	Close,
}
sqlb::bindable!(TodoStatus);
// endregion: Todo Types

// region:    TodoMac
pub struct TodoMac;

impl TodoMac {
	// const TABLE: &'static str = "todo";
	const TABLE: &'static str = "raw_data";
	// const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
	const COLUMNS: &'static [&'static str] = &["id", "timestamp", "value"];
}

/// Todo Model Access Controller
impl TodoMac {

	pub async fn get(db: &Db, id: i64) -> Result<Todo, model::Error> {
		let sb = sqlb::select()
			.table(Self::TABLE)
			.columns(Self::COLUMNS);
			// .and_where_eq("id", id);

		let result = sb.fetch_one(db).await;

		handle_fetch_one_result(result, Self::TABLE, id)
	}

	pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
		// let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS).order_by("!id");
		let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

		// execute the query
		let todos = sb.fetch_all(db).await?;


		Ok(todos)
	}
}
// endregion: TodoMac

// region:    Utils
fn handle_fetch_one_result(
	result: Result<Todo, sqlx::Error>,
	typ: &'static str,
	id: i64,
) -> Result<Todo, model::Error> {
	result.map_err(|sqlx_error| match sqlx_error {
		sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id.to_string()),
		other => model::Error::SqlxError(other),
	})
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
// endregion: Test

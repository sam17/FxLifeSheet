use super::db::Db;
use crate::model;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct RawDataObj {
	pub timestamp: i64,
	pub value: String,
}

// #[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// #[sqlx(type_name = "todo_status_enum")]
// #[sqlx(rename_all = "lowercase")]
// pub enum TodoStatus {
// 	Open,
// 	Close,
// }
// sqlb::bindable!(TodoStatus);

pub struct RawData;

impl RawData {
	const TABLE: &'static str = "raw_data";
	const COLUMNS: &'static [&'static str] = &["timestamp", "value"];
}

/// Todo Model Access Controller
impl RawData {

	pub async fn get_by_key(db: &Db, key: String) -> Result<Vec<RawDataObj>, model::Error> {
		let sb = sqlb::select()
			.table(Self::TABLE)
			.columns(Self::COLUMNS).and_where_eq("key", key);

		let data_by_key = sb.fetch_all(db).await?;
		Ok(data_by_key)
	}

	// pub async fn getAll(db: &Db, id: i64) -> Result<RawData, model::Error> {
	// 	let sb = sqlb::select()
	// 		.table(Self::TABLE)
	// 		.columns(Self::COLUMNS);
	// 		// .and_where_eq("id", id);
	//
	// 	let result = sb.fetch_one(db).await;
	//
	// 	handle_fetch_one_result(result, Self::TABLE, id)
	// }

	pub async fn list(db: &Db) -> Result<Vec<RawDataObj>, model::Error> {
		let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

		// execute the query
		let raw_data = sb.fetch_all(db).await?;
		Ok(raw_data)
	}
}
// endregion: TodoMac

// region:    Utils
// fn handle_fetch_one_result(
// 	result: Result<RawData, sqlx::Error>,
// 	typ: &'static str,
// 	id: i64,
// ) -> Result<RawData, model::Error> {
// 	result.map_err(|sqlx_error| match sqlx_error {
// 		sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id.to_string()),
// 		other => model::Error::SqlxError(other),
// 	})
// }
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
// endregion: Test

use serde::{Deserialize, Serialize};
use crate::models::core::db::Db;
use crate::models::Error;

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

impl RawData {

	pub async fn get_by_key(db: &Db, key: String) -> Result<Vec<RawDataObj>, Error> {
		let sb = sqlb::select()
			.table(Self::TABLE)
			.columns(Self::COLUMNS).and_where_eq("key", key);

		let data_by_key = sb.fetch_all(db).await?;
		Ok(data_by_key)
	}

	pub async fn list(db: &Db) -> Result<Vec<RawDataObj>, Error> {
		let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

		// execute the query
		let raw_data = sb.fetch_all(db).await?;
		Ok(raw_data)
	}
}


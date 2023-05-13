use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;


#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct RawDataObj {
	pub timestamp: i64,
	pub value: String,
}

#[derive(Fields, Debug, Clone, Serialize, Deserialize)]
pub struct PublishableDataObj {
	pub timestamp: i64,
	pub question_key: String,
	pub value: String,
	pub source: String,
	pub user_id: i32,
}

impl<'r> FromRow<'r, PgRow> for PublishableDataObj {
	fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
		Ok(PublishableDataObj {
			timestamp: row.try_get("timestamp")?,
			question_key: row.try_get("key")?,
			value: row.try_get("value")?,
			source: row.try_get("source")?,
			user_id: row.try_get("user_id")?,
		})
	}
}
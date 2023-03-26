use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct RawDataObj {
	pub timestamp: i64,
	pub value: String,
}

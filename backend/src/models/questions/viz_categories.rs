use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct VizCategoriesObj {
	pub id: i32,
	pub name: String,
	pub priority: i32,
	pub description: String,
}

use super::db::Db;
use crate::model;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct VizCategoriesObj {
	pub id: i32,
	pub name: String,
	pub priority: i32,
	pub description: String,
}

pub struct VizCategories;

impl VizCategories {
    const TABLE: &'static str = "category";
    const COLUMNS: &'static [&'static str] = &["id", "name", "priority", "description"];
}

impl VizCategories {
	pub async fn get_all_categories(db: &Db) -> Result<Vec<VizCategoriesObj>, model::Error> {
		let sb = sqlb::select()	
			.table(Self::TABLE)
			.columns(Self::COLUMNS);

		let viz_categories_list = sb.fetch_all(db).await?;
		Ok(viz_categories_list)
	}
}

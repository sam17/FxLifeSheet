use super::db::Db;
use crate::model;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct VizQuestionsObj {
    pub key: String,
    pub question: String,
	pub question_type : String,
	pub max_value: i32,
	pub min_value: i32,
}

pub struct VizQuestions;

impl VizQuestions {
    const TABLE: &'static str = "questions";
    const COLUMNS: &'static [&'static str] = &["key", "question", "question_type", "max_value", "min_value"];
}

impl VizQuestions {

	// get all where is_active = true
	pub async fn get_visible_list(db: &Db) -> Result<Vec<VizQuestionsObj>, model::Error> {
		let sb = sqlb::select()	
			.table(Self::TABLE)
			.columns(Self::COLUMNS)
			.and_where_eq("is_visible_in_visualizer", true);

		let viz_questions_list = sb.fetch_all(db).await?;
		Ok(viz_questions_list)
	}
}

use super::db::Db;
use crate::model;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct VizQuestionsObj {
    pub key: String,
    pub question: String,
    pub question_type: String,
    pub max_value: Option<i32>,
    pub min_value: Option<i32>,
	pub buttons: Option<String>,
    pub is_positive: bool,
    pub is_reverse: bool,
    pub display_name: String,
}

pub struct VizQuestions;

impl VizQuestions {
    const TABLE: &'static str = "questions";
    const COLUMNS: &'static [&'static str] =
        &["key", "question", "question_type", "max_value", "min_value", "buttons"
        , "is_positive", "is_reverse", "display_name"];
}

impl VizQuestions {
    pub async fn get_questions_with_query(
        db: &Db,
        category: String,
        is_visible: bool,
    ) -> Result<Vec<VizQuestionsObj>, model::Error> {
        let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

		if is_visible {
			sb = sb.and_where_eq("is_visible_in_visualizer", true);
		}

		if category != "" {
			sb = sb.and_where_eq("category", category);
		}	

        let viz_questions_list = sb.fetch_all(db).await?;
        Ok(viz_questions_list)
    }
}

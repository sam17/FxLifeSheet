use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VizQuestionsQuery {
    pub category: Option<String>,
    pub is_visible: Option<bool>,
}

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

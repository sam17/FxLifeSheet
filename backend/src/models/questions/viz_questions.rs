use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Postgres, Row, Type};
use sqlx::postgres::{PgRow, PgValueRef};
use crate::models::questions::question_options::QuestionOption;


#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
pub struct QuestionKey(pub String);



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: i32,
    pub key: String,
    pub question: String,
    pub answer_type: String,
    pub parent_question: Option<String>,
    pub parent_question_option: Option<String>,
    pub category: Option<i32>,
    pub max_value: Option<i32>,
    pub min_value: Option<i32>,
    pub show: bool,
    pub display_name: String,
    pub is_positive: bool,
    pub cadence: String,
    pub command: Option<String>,
    pub graph_type: String,
    pub question_options: Option<Vec<QuestionOption>>,
}

impl Question {

    pub fn set_question_options(&mut self, question_options: Vec<QuestionOption>) {
        self.question_options = Option::from(question_options);
    }
}


impl<'r> FromRow<'r, PgRow> for Question {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Question {
            id: row.try_get("id")?,
            key: row.try_get("key")?,
            question: row.try_get("question")?,
            answer_type: row.try_get("answer_type")?,
            parent_question: row.try_get("parent_question").ok(),
            parent_question_option: row.try_get("parent_question_option").ok(),
            category: row.try_get("category").ok(),
            max_value: row.try_get("max_value").ok(),
            min_value: row.try_get("min_value").ok(),
            show: row.try_get("show")?,
            display_name: row.try_get("display_name")?,
            is_positive: row.try_get("is_positive")?,
            cadence: row.try_get("cadence")?,
            command: row.try_get("command")?,
            graph_type: row.try_get("graph_type")?,
            question_options: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct VizQuestionsQuery {
    pub category: Option<String>,
    pub is_visible: Option<bool>,
    pub command: Option<String>,
}
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuestionOption {
    id: i32,
    name: String,
    question_key: String,
}

impl QuestionOption {
    pub fn new(id: i32, name: String, question_key: String) -> Self {
        QuestionOption {
            id,
            name,
            question_key,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn question_key(&self) -> &str {
        &self.question_key
    }
}

impl<'r> FromRow<'r, PgRow> for QuestionOption {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(QuestionOption {
            // Replace with the names and types of your fields in the VizQuestionOptions struct
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            question_key: row.try_get("question_key")?,
        })
    }
}

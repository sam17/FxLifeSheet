use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Postgres, Row, Type};
use sqlx::postgres::{PgRow, PgValueRef};

// #[derive(Serialize, Deserialize)]
// pub struct VizQuestionsQuery {
//     pub category: Option<String>,
//     pub is_visible: Option<bool>,
// }
//
// #[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
// pub struct VizQuestionsObj {
//     pub key: String,
//     pub question: String,
//     pub question_type: String,
//     pub max_value: Option<i32>,
//     pub min_value: Option<i32>,
// 	pub buttons: Option<String>,
//     pub is_positive: bool,
//     pub is_reverse: bool,
//     pub display_name: String,
// }

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
pub struct QuestionKey(String);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    id: i32,
    key: QuestionKey,
    question: String,
    answer_type: String,
    parent_question: Option<QuestionKey>,
    parent_question_option: Option<String>,
    category: Option<i32>,
    max: Option<i32>,
    min: Option<i32>,
    show: bool,
    display_name: String,
    is_positive: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuestionOption {
    id: i32,
    name: String,
    question_key: QuestionKey,
}

impl<'r> FromRow<'r, PgRow> for Question {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Question {
            id: row.try_get("id")?,
            key: QuestionKey(row.try_get("key")?),
            question: row.try_get("question")?,
            answer_type: row.try_get("answer_type")?,
            parent_question: row.try_get("parent_question").ok(),
            parent_question_option: row.try_get("parent_question_option").ok(),
            category: row.try_get("category").ok(),
            max: row.try_get("max").ok(),
            min: row.try_get("min").ok(),
            show: row.try_get("show")?,
            display_name: row.try_get("display_name")?,
            is_positive: row.try_get("is_positive")?,
        })
    }
}

impl Type<Postgres> for QuestionKey {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as Type<Postgres>>::compatible(ty)
    }
}

impl<'r> Decode<'r, Postgres> for QuestionKey {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let inner_value: String = Decode::decode(value)?;
        Ok(QuestionKey(inner_value))
    }
}

#[derive(Debug, Deserialize)]
pub struct VizQuestionsQuery {
    pub category: Option<String>,
    pub is_visible: Option<bool>,
}
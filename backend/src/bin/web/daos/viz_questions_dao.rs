use std::mem::size_of;
use sqlx::postgres::PgRow;
use models::models::questions::viz_questions::Question;
use models::models::questions::question_options::QuestionOption;
use crate::daos::viz_categories_dao::VizCategories;
use crate::utils::db::Db;
use crate::utils::error::ModelError;

pub struct VizQuestions;

impl VizQuestions {
    const TABLE: &'static str = "questions";
    const COLUMNS: &'static [&'static str] = &[
        "id",
        "key",
        "question",
        "answer_type",
        "max_value",
        "min_value",
        "show",
        "is_positive",
        "display_name",
        "category",
        "command",
        "cadence",
        "graph_type"
    ];
}

impl VizQuestions {
    pub async fn get_questions_with_query(
        db: &Db,
        category_name: Option<String>,
        is_visible: bool,
        command: Option<String>,
    ) -> Result<Vec<Question>, ModelError> {
        let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

        if is_visible {
            sb = sb.and_where_eq("show", true);
        }

        if let Some(cat_name) = category_name {
            let category_id = VizCategories::get_id_by_name(&db, &cat_name).await?;
            sb = sb.and_where_eq("category", category_id);
        }

        if let Some(cmd) = command {
            sb = sb.and_where_eq("command", cmd);
        }

        let mut questions_list: Vec<Question> = sb.fetch_all(db).await?;

        // Fetch question options for each question and update the question struct
        for question in questions_list.iter_mut() {
            let question_options:Vec<QuestionOption> =
                VizQuestionOptions::get_options_for_question
                    (db, &question.key).await?;
            question.set_question_options(question_options);
        }

        Ok(questions_list)
    }

    pub async fn get_child_questions(
        db: &Db,
        parent_question_key: &str,
        parent_question_option: &str,
    ) -> Result<Vec<Question>, ModelError> {
        let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);
        sb = sb.and_where_eq("parent_question", parent_question_key)
            .and_where_eq("parent_question_option", parent_question_option);

        let questions_list: Vec<Question> = sb.fetch_all(db).await?;
        Ok(questions_list)
    }

}

pub struct VizQuestionOptions;

impl VizQuestionOptions{
    const TABLE: &'static str = "options";
    const COLUMNS: &'static [&'static str] = &[
        "id",
        "question_key",
        "name",
    ];
}

impl VizQuestionOptions{
    pub async fn get_options_for_question(
        db: &Db,
        question_key: &str,
    ) -> Result<Vec<QuestionOption>, ModelError> {
        let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);
        sb = sb.and_where_eq("question_key", question_key);

        let options_list: Vec<QuestionOption> = sb.fetch_all(db).await?;
        Ok(options_list)
    }
}

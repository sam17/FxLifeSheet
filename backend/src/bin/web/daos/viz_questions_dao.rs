use std::mem::size_of;
use models::models::questions::viz_questions::Question;
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
        "max",
        "min",
        "show",
        "is_positive",
        "display_name",
        "category",
        "command"
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

        let questions_list = sb.fetch_all(db).await?;
        Ok(questions_list)
    }

    pub async fn get_questions_for_command(
        db: &Db,
        command: String,
    ) -> Result<Vec<Question>, ModelError> {
        let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);
        sb = sb.and_where_eq("command", command);

        let questions_list = sb.fetch_all(db).await?;
        // print size of questions_list
        println!("questions_list size: {}", size_of::<Vec<Question>>());
        Ok(questions_list)
    }
}


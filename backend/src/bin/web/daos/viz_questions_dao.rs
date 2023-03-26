use models::models::questions::viz_questions::VizQuestionsObj;
use crate::utils::db::Db;
use crate::utils::error::ModelError;

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
    ) -> Result<Vec<VizQuestionsObj>, ModelError> {
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

use models::models::questions::viz_questions::Question;
use crate::daos::viz_categories_dao::VizCategories;
use crate::utils::db::Db;
use crate::utils::error::ModelError;

pub struct VizQuestions;

impl VizQuestions {
    const TABLE: &'static str = "questions";
    const COLUMNS: &'static [&'static str] = &[
        "key",
        "question",
        "answer_type",
        "max",
        "min",
        "show",
        "is_positive",
        "display_name",
    ];
}

// impl VizQuestions {
//     pub async fn get_questions_with_query(
//         db: &Db,
//         category: String,
//         is_visible: bool,
//     ) -> Result<Vec<Question>, ModelError> {
//         let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);
//
//         if is_visible {
//             sb = sb.and_where_eq("show", true);
//         }
//
//         if !category.is_empty() {
//             sb = sb.and_where_eq("category", category);
//         }
//
//         let questions_list = sb.fetch_all(db).await?;
//         Ok(questions_list)
//     }
// }

// impl VizQuestions {
//     pub async fn get_questions_with_query(
//         db: &Db,
//         category: Option<i32>,
//         is_visible: bool,
//     ) -> Result<Vec<Question>, ModelError> {
//         let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);
//
//         if is_visible {
//             sb = sb.and_where_eq("show", true);
//         }
//
//         if let Some(cat_id) = category {
//             sb = sb.and_where_eq("category", cat_id);
//         }
//
//         let questions_list = sb.fetch_all(db).await?;
//         Ok(questions_list)
//     }
// }

impl VizQuestions {
    pub async fn get_questions_with_query(
        db: &Db,
        category_name: Option<String>,
        is_visible: bool,
    ) -> Result<Vec<Question>, ModelError> {
        let mut sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

        if is_visible {
            sb = sb.and_where_eq("show", true);
        }

        if let Some(cat_name) = category_name {
            let category_id = VizCategories::get_id_by_name(&db, &cat_name).await?;
            sb = sb.and_where_eq("category", category_id);
        }

        let questions_list = sb.fetch_all(db).await?;
        Ok(questions_list)
    }
}


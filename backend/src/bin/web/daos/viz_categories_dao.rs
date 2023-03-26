use models::models::questions::viz_categories::VizCategoriesObj;
use crate::utils::db::Db;
use crate::utils::error::ModelError;

pub struct VizCategories;

impl VizCategories {
    const TABLE: &'static str = "category";
    const COLUMNS: &'static [&'static str] = &["id", "name", "priority", "description"];
}

impl VizCategories {
    pub async fn get_all_categories(db: &Db) -> Result<Vec<VizCategoriesObj>, ModelError> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS);

        let viz_categories_list = sb.fetch_all(db).await?;
        Ok(viz_categories_list)
    }
}

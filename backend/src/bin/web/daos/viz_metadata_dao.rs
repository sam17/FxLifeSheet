use models::models::core::viz_metadata::VizMetadataObj;
use crate::utils::db::Db;
use crate::utils::error::ModelError;


pub struct VizMetadata;

impl VizMetadata {
    const TABLE: &'static str = "metadata";
    const COLUMNS: &'static [&'static str] = &["key", "value"];
}

impl VizMetadata {
    pub async fn get_by_key(db: &Db, key: String) -> Result<VizMetadataObj, ModelError> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS).and_where_eq("key", &key);

        let result = sb.fetch_one(db).await;

        handle_fetch_one_result(result, Self::TABLE, &key)
    }

    pub async fn list(db: &Db) -> Result<Vec<VizMetadataObj>, ModelError> {
        let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

        // execute the query
        let viz_metadata_list = sb.fetch_all(db).await?;
        Ok(viz_metadata_list)
    }
}


//region:    Utils
fn handle_fetch_one_result(
    result: Result<VizMetadataObj, sqlx::Error>,
    typ: &'static str,
    key: &String,
) -> Result<VizMetadataObj, ModelError> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => ModelError::EntityNotFound(typ, key.to_string()),
        other => ModelError::SqlxError(other),
    })
}
//endregion: Utils
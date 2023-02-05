use super::db::Db;
use crate::model;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct VizMetadataObj {
    pub key: String,
    pub value: String,
}

pub struct VizMetadata;

impl VizMetadata {
    const TABLE: &'static str = "metadata";
    const COLUMNS: &'static [&'static str] = &["key", "value"];
}

impl VizMetadata {
    pub async fn get_by_key(db: &Db, key: String) -> Result<VizMetadataObj, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS).and_where_eq("key", &key);

    	let result = sb.fetch_one(db).await;

    	handle_fetch_one_result(result, Self::TABLE, &key)
    }

	pub async fn list(db: &Db) -> Result<Vec<VizMetadataObj>, model::Error> {
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
) -> Result<VizMetadataObj, model::Error> {
	result.map_err(|sqlx_error| match sqlx_error {
		sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, key.to_string()),
		other => model::Error::SqlxError(other),
	})
}
//endregion: Utils
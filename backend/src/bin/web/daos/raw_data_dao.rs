use models::models::collector::raw_data::RawDataObj;
use crate::utils::db::Db;
use crate::utils::error::ModelError;

pub struct RawData;

impl RawData {
    const TABLE: &'static str = "raw_data";
    const COLUMNS: &'static [&'static str] = &["timestamp", "value"];
}

impl RawData {

    pub async fn get_by_key(db: &Db, key: String) -> Result<Vec<RawDataObj>, ModelError> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS).and_where_eq("key", key);

        let data_by_key = sb.fetch_all(db).await?;
        Ok(data_by_key)
    }

    pub async fn list(db: &Db) -> Result<Vec<RawDataObj>, ModelError> {
        let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

        // execute the query
        let raw_data = sb.fetch_all(db).await?;
        Ok(raw_data)
    }
}


use sqlx::query;
use models::models::collector::raw_data::{PublishableDataObj, RawDataObj};
use crate::utils::db::Db;
use crate::utils::error::ModelError;


pub struct RawData;

impl RawData {
    const TABLE: &'static str = "raw_data";
    const COLUMNS: &'static [&'static str] = &["timestamp", "key", "value", "source", "user_id"];
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

    // function to post data to the database in PublishableDataObj format
    pub async fn insert_data(db: &Db, data: &PublishableDataObj) -> Result<PublishableDataObj, ModelError> {
        let result = query!(
        r#"
        INSERT INTO raw_data (timestamp, key, value, source, user_id)
        VALUES ($1, $2, $3, $4, $5) RETURNING timestamp, key, value, source, user_id
        "#,
        data.timestamp,
        &data.question_key,
        &data.value,
        &data.source,
        data.user_id).fetch_one(db).await?;

        let response = PublishableDataObj {
            timestamp: result.timestamp.unwrap_or_default(),
            question_key: result.key.unwrap_or_default(),
            value: result.value.unwrap_or_default(),
            source: result.source.unwrap_or_default(),
            user_id: result.user_id,
        };

        Ok(response)
    }

}

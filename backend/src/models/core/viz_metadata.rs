use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct VizMetadataObj {
    pub key: String,
    pub value: String,
}


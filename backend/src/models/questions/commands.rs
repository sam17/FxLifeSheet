use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Commands {
    name: String,
    description: String,
    schedule: String,
}

impl Commands {
    fn new(name: String, description: String, schedule: String) -> Self {
        Self {
            name,
            description,
            schedule,
        }
    }
}

impl<'r> FromRow<'r, PgRow> for Commands {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let name: String = row.try_get("name")?;
        let description: String = row.try_get("description")?;
        let schedule: String = row.try_get("schedule")?;

        Ok(Commands {
            name,
            description,
            schedule,
        })
    }
}
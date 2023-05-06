use crate::utils::db::Db;
use crate::utils::error::ModelError;
use models::models::questions::commands::Commands;


pub struct BotCommands;

impl BotCommands {
    const TABLE: &'static str = "commands";
    const COLUMNS: &'static [&'static str] = &["name", "description", "schedule"];
}

impl BotCommands {

    pub async fn get_by_name(db: &Db, name: String) -> Result<Vec<Commands>, ModelError> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS).and_where_eq("name", name);

        let command_by_name = sb.fetch_all(db).await?;
        Ok(command_by_name)
    }

    pub async fn list(db: &Db) -> Result<Vec<Commands>, ModelError> {
        let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS);

        // execute the query
        let commands = sb.fetch_all(db).await?;
        Ok(commands)
    }
}
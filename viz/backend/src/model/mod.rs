mod db;
mod raw_data_dao;
mod viz_metadata_dao;
mod viz_questions_dao;
mod viz_categories_dao;

// re-export
pub use db::init_db;
pub use db::Db;
pub use raw_data_dao::RawData;
pub use viz_metadata_dao::VizMetadata;
pub use viz_questions_dao::VizQuestions;
pub use viz_categories_dao::VizCategories;

// region:    Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Entity Not Found - {0}[{1}] ")]
	EntityNotFound(&'static str, String),

	#[error(transparent)]
	Sqlx(#[from] sqlx::Error),

	#[error(transparent)]
	Io(#[from] std::io::Error),
}

// endregion: Error

use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;
use crate::daos::viz_metadata_dao::VizMetadata;
use crate::utils::db::Db;
use crate::utils::filter_utils;

pub fn viz_metadata_rest_filters(
    base_path: &'static str,
    db: &Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let data_path = warp::path(base_path).and(warp::path("metadata"));
    // let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));
    let common = filter_utils::with_db(db.clone());

    let get = data_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(metadata_get_by_key);

    // LIST viz_metadata `GET metadata/`
    let list = data_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(metadata_list);

    get.or(list)
}

async fn metadata_get_by_key(db: Arc<Db>, key: String) -> Result<Json, warp::Rejection> {
    let data = VizMetadata::get_by_key(&db,  key).await?;
    let response = json!({ data.key: data.value });
    Ok(warp::reply::json(&response))
}

async fn metadata_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let metadata_list = VizMetadata::list(&db).await?;
    // convert metadata_list to a map
    let mut metadata_map = std::collections::HashMap::new();
    for metadata in metadata_list {
        metadata_map.insert(metadata.key, metadata.value);
    }
    // convert metadata_map to json without extra key
    let response = json!(metadata_map);
    Ok(warp::reply::json(&response))
}

use std::sync::Arc;

use crate::config::Config;
use crate::csv_loader::Posicion;
use crate::database::{model::University, DbPool};
use axum::{
    extract::{Query, State},
    routing::{get, put},
    Json, Router,
};
use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::database::Database;

use self::extractors::DestinationsData;

pub mod extractors;

async fn get_unis(
    State(db): State<Arc<DbPool>>,
) -> Result<Json<Vec<University>>, (StatusCode, String)> {
    (&*db)
        .get_all_universities()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Debug, serde::Deserialize)]
struct DestinosParams {
    user: String,
	codigo_erasmus: usize,
    nivel_estudios: Option<usize>,
    plazas: Option<usize>,
    meses: Option<usize>,
    idioma: Option<usize>,
    observaciones: Option<usize>,
    #[serde(default)]
    header: bool,
}
async fn put_destinos(Query(params): Query<DestinosParams>, csv: DestinationsData) {
    println!("Params: {params:?}");
    println!("CSV: {csv:?}");
}

pub async fn api(_config: &Config) -> sqlx::Result<Router> {
    let db = SqlitePool::connect("sqlite:db.sqlite").await.unwrap();
    let db = DbPool::new(db);
    Ok(Router::new()
        .route("/unis", get(get_unis))
        .route("/destination", put(put_destinos))
        .with_state(Arc::new(db)))
}

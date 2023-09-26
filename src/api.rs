use std::io::{BufReader, Cursor};
use std::sync::Arc;

use crate::config::Config;
use crate::csv_loader::{Posicion, load_csv};
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

impl From<&DestinosParams> for Posicion {
    fn from(value: &DestinosParams) -> Self {
        Self {
            codigo_erasmus: value.codigo_erasmus,
            nivel_estudios: value.nivel_estudios,
            plazas: value.plazas,
            meses: value.meses,
            idioma: value.idioma,
            observaciones: value.observaciones,
            header: value.header,
        }
    }
}

async fn put_destinos(State(db): State<Arc<DbPool>>, Query(params): Query<DestinosParams>, csv: DestinationsData) {
    
    println!("Params: {params:?}");
    println!("CSV: {csv:?}");
    let mut db = db.as_ref();
    let res = load_csv(&mut db, (&params).into(), &params.user, Cursor::new(csv.csv)).await;
}

pub async fn api(_config: &Config) -> sqlx::Result<Router> {
    let db = SqlitePool::connect("sqlite:db.sqlite").await.unwrap();
    let db = DbPool::new(db);
    Ok(Router::new()
        .route("/unis", get(get_unis))
        .route("/destination", put(put_destinos))
        .with_state(Arc::new(db)))
}

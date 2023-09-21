use std::{borrow::Cow, fs::File, sync::Arc};

use axum::{Server, Router, routing::get, body::Body, Json, extract::State};
use config::Config;
use database::model::University;
use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::database::Database;

mod database;
mod config;
mod csv_loader;

async fn get_unis(State(db): State<Arc<Database>>) -> Result<Json<Vec<University>>, (StatusCode, String)> {
    db.get_all_universities().await.map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[tokio::main]
async fn main() {
    let config_path = std::env::var("ERASMUS_CONFIG").unwrap_or_else(|_| "config.json".into());
    let config = serde_json::from_reader::<_, Config>(File::open(config_path).unwrap()).unwrap();
    dbg!(&config);
    let db = SqlitePool::connect("sqlite:db.sqlite").await.unwrap();
    let db = Database::new(db);

    let router = Router::new().route("/api/unis", get(get_unis)).with_state(Arc::new(db));

    Server::bind(&config.addr).serve(router.into_make_service()).await.unwrap();
    // let info = Posicion {
    //     // pais: Some(0),
    //     // ciudad: Some(1),
    //     // universidad: Some(2),
    //     codigo_erasmus: 3,
    //     nivel_estudios: Some(5),
    //     plazas: Some(6),
    //     meses: Some(7),
    //     idioma: Some(8),
    //     observaciones: Some(9),
    // };
    
    // load_csv(&db, info, "Juan", "destinos/Erasmus - Destinos Juan.csv")
    //     .await
    //     .unwrap();
}

#[derive(Debug, Clone)]
pub struct ErasmusCode<'a> {
    pais: Cow<'a, str>,
    region: Cow<'a, str>,
    universidad: u32,
}

impl std::fmt::Display for ErasmusCode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{:02}", self.pais, self.region, self.universidad)
    }
}

impl<'a> TryFrom<&'a str> for ErasmusCode<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (pais, ciu_uni) = value.split_once(' ').ok_or(())?;
        let i = ciu_uni
            .char_indices()
            .find(|(_, c)| c.is_numeric())
            .ok_or(())?
            .0;
        let ciudad = ciu_uni[..i].trim();
        let uni = ciu_uni[i..].trim().parse().map_err(|_| ())?;
        Ok(Self {
            pais: pais.into(),
            region: ciudad.into(),
            universidad: uni,
        })
    }
}

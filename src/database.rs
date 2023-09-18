use sqlx::{SqlitePool, query_as, query};
use thiserror::Error;

use crate::ErasmusCode;

pub struct Database{
	db: SqlitePool
}

impl Database {
    pub const fn new(db: SqlitePool) -> Self { Self { db } }
}

#[derive(Debug, Error)]
pub enum GetUniCityError {
	#[error(transparent)]
	SqlxError(#[from] sqlx::Error),
	#[error("Solvable problem")]
	SolvableProblem(GetUniCitySolvableProblem)
}

impl From<GetUniCitySolvableProblem> for GetUniCityError {
    fn from(value: GetUniCitySolvableProblem) -> Self {
        Self::SolvableProblem(value)
    }
}

#[derive(Debug)]
pub struct UniCity {
	pub uni: Option<String>,
	pub city: String
}

#[derive(Debug)]
pub struct Country {
	pub erasmus_code: String,
	pub name: String,
	pub iso_code: String
}

#[derive(Debug)]
pub enum GetUniCitySolvableProblem {
	MultipleOptions(Vec<UniCity>),
	CountryWrong(Country)
}

impl Database{
	pub async fn get_uni_city(&self, uni: &ErasmusCode<'_>) -> Result<UniCity, GetUniCityError> {
		let mut res = query_as!(UniCity, "SELECT ciudad as city, nombre as uni FROM Universidad WHERE numero = ? AND pais = ? AND region = ?", uni.universidad, uni.pais, uni.region).fetch_all(&self.db).await?;
		match res.len() {
			0 => Err(self.find_solutions(uni).await?.into()),
			1 => Ok(res.remove(0)),
			_ => Err(GetUniCitySolvableProblem::MultipleOptions(res).into())
		}
	}
	async fn find_solutions(&self, uni: &ErasmusCode<'_>) -> Result<GetUniCitySolvableProblem, sqlx::Error> {
		if let Some(country) = query_as!(Country, "SELECT p.codigo as erasmus_code, p.nombre as name, p.codigo_iso as iso_code FROM Universidad u, Pais p WHERE u.pais = p.codigo AND u.region = ? AND u.numero = ?", uni.region, uni.universidad).fetch_optional(&self.db).await? {
			return Ok(GetUniCitySolvableProblem::CountryWrong(country))
		}

		todo!("{uni}")
	}
}
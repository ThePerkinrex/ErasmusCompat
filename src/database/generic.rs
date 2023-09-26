use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use sqlx::{query, query_as, Executor, Pool, Sqlite, SqlitePool};
use thiserror::Error;

use crate::{
    database::model::{Country, CountryId, UniCity},
    ErasmusCode,
};

use super::model::{CityId, University};

use super::DB;

#[derive(Debug, Error)]
pub enum GetUniCityError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Solvable problem")]
    SolvableProblem(GetUniCitySolvableProblem),
}

impl From<GetUniCitySolvableProblem> for GetUniCityError {
    fn from(value: GetUniCitySolvableProblem) -> Self {
        Self::SolvableProblem(value)
    }
}

#[derive(Debug, serde::Serialize)]
pub enum GetUniCitySolvableProblem {
    MultipleOptions(Vec<UniCity>),
    CountryWrong(Country),
    UniDoesntExist { city: CityId, uni: u32 },
}

#[async_trait::async_trait]
pub trait Database {
    async fn get_uni_city(&mut self, uni: &ErasmusCode<'_>) -> Result<UniCity, GetUniCityError>;
    async fn get_all_universities(&mut self) -> sqlx::Result<Vec<University>>;

    type TransactionDb<'a>: Database
    where
        Self: 'a;
    type Transaction<'a>: DerefMut + Deref<Target = Self::TransactionDb<'a>> + TransactionOps
    where
        Self: 'a;
    async fn begin<'a>(&'a mut self) -> sqlx::Result<Self::Transaction<'a>>;
}

#[async_trait::async_trait]
impl<T> Database for T
where
    for<'a> &'a mut T: Executor<'a, Database = DB> + sqlx::Acquire<'a, Database = DB>,
    T: Send,
{
    async fn get_uni_city(&mut self, uni: &ErasmusCode<'_>) -> Result<UniCity, GetUniCityError> {
        let mut res = query_as!(UniCity, "SELECT ciudad as city, nombre as uni FROM Universidad WHERE numero = ? AND pais = ? AND region = ?", uni.universidad, uni.pais, uni.region).fetch_all(&mut *self).await?;
        match res.len() {
            0 => Err(async move {
				if let Some(country) = query_as!(Country, "SELECT p.codigo as erasmus_code, p.nombre as name, p.codigo_iso as iso_code FROM Universidad u, Pais p WHERE u.pais = p.codigo AND u.region = ? AND u.numero = ?", uni.region, uni.universidad).fetch_optional(&mut *self).await? {
					return Ok::<_, sqlx::Error>(GetUniCitySolvableProblem::CountryWrong(country))
				}
				let mut names = query!("SELECT nombre FROM Ciudad WHERE pais = ? AND region = ?", uni.pais, uni.region).fetch_all(&mut *self).await?.into_iter().map(|x| x.nombre).collect::<Vec<_>>();
				let country_exists = query!("SELECT COUNT(*) as n FROM Pais WHERE codigo = ?", uni.pais).fetch_one(&mut *self).await?.n != 0;
				let country = CountryId::new(uni.pais.to_string(), country_exists);
				let city = match names.len() {
					0 => CityId::new_inexistant(uni.region.to_string(), country),
					1 => CityId::new_single(uni.region.to_string(), names.remove(0), country),
					_ => CityId::new_multiple(uni.region.to_string(), names, country)
				};
				Ok(GetUniCitySolvableProblem::UniDoesntExist { city, uni: uni.universidad })
			}.await?.into()),
            1 => Ok(res.remove(0)),
            _ => Err(GetUniCitySolvableProblem::MultipleOptions(res).into()),
        }
    }

    async fn get_all_universities(&mut self) -> sqlx::Result<Vec<University>> {
        query_as!(University, "SELECT nombre as name, numero as number, region, pais as country, ciudad as city, lat, lon FROM Universidad").fetch_all(self).await
    }

    type TransactionDb<'a> = <Self::Transaction<'a> as Deref>::Target where Self: 'a;
    type Transaction<'a> = sqlx::Transaction<'a, DB> where Self: 'a;
    async fn begin<'a>(&'a mut self) -> sqlx::Result<Self::Transaction<'a>> {
        sqlx::Acquire::begin(self).await
    }
}

pub struct DbPool {
    db: Pool<DB>,
}

#[async_trait::async_trait]
impl<'l> Database for &'l DbPool {
    async fn get_uni_city(&mut self, uni: &ErasmusCode<'_>) -> Result<UniCity, GetUniCityError> {
        self.db.acquire().await?.get_uni_city(uni).await
    }

    async fn get_all_universities(&mut self) -> sqlx::Result<Vec<University>> {
        self.db.acquire().await?.get_all_universities().await
    }

    type TransactionDb<'a> = <Self::Transaction<'a> as Deref>::Target where Self: 'a;
    type Transaction<'a> = sqlx::Transaction<'a, DB> where Self: 'a;
    async fn begin<'a>(&'a mut self) -> sqlx::Result<Self::Transaction<'a>> {
        self.db.begin().await
    }
}

impl DbPool {
    pub const fn new(db: Pool<DB>) -> Self {
        Self { db }
    }

    pub async fn begin(&self) -> sqlx::Result<sqlx::Transaction<'_, DB>> {
        self.db.begin().await
    }
}

#[async_trait::async_trait]
pub trait TransactionOps {
    async fn commit(self) -> sqlx::Result<()>;
    async fn rollback(self) -> sqlx::Result<()>;
}

#[async_trait::async_trait]
impl<'a> TransactionOps for sqlx::Transaction<'a, DB> {
    async fn commit(self) -> sqlx::Result<()> {
        self.commit().await
    }
    async fn rollback(self) -> sqlx::Result<()> {
        self.rollback().await
    }
}

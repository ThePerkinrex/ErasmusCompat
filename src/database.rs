use sqlx::Sqlite;

mod generic;
pub mod model;

pub type DB = Sqlite;

pub use generic::*;

// pub struct Database {
//     db: SqlitePool,
// }

// impl Database {
//     pub const fn new(db: SqlitePool) -> Self {
//         Self { db }
//     }
// }

// #[derive(Debug, Error)]
// pub enum GetUniCityError {
//     #[error(transparent)]
//     SqlxError(#[from] sqlx::Error),
//     #[error("Solvable problem")]
//     SolvableProblem(GetUniCitySolvableProblem),
// }

// impl From<GetUniCitySolvableProblem> for GetUniCityError {
//     fn from(value: GetUniCitySolvableProblem) -> Self {
//         Self::SolvableProblem(value)
//     }
// }

// #[derive(Debug)]
// pub enum GetUniCitySolvableProblem {
//     MultipleOptions(Vec<UniCity>),
//     CountryWrong(Country),
//     UniDoesntExist {
//         city: CityId,
// 		uni: u32
//     },
// }

// impl Database {
//     pub async fn get_uni_city(&self, uni: &ErasmusCode<'_>) -> Result<UniCity, GetUniCityError> {
//         let mut res = query_as!(UniCity, "SELECT ciudad as city, nombre as uni FROM Universidad WHERE numero = ? AND pais = ? AND region = ?", uni.universidad, uni.pais, uni.region).fetch_all(&self.db).await?;
//         match res.len() {
//             0 => Err(self.find_solutions(uni).await?.into()),
//             1 => Ok(res.remove(0)),
//             _ => Err(GetUniCitySolvableProblem::MultipleOptions(res).into()),
//         }
//     }
//     async fn find_solutions(
//         &self,
//         uni: &ErasmusCode<'_>,
//     ) -> Result<GetUniCitySolvableProblem, sqlx::Error> {
//         if let Some(country) = query_as!(Country, "SELECT p.codigo as erasmus_code, p.nombre as name, p.codigo_iso as iso_code FROM Universidad u, Pais p WHERE u.pais = p.codigo AND u.region = ? AND u.numero = ?", uni.region, uni.universidad).fetch_optional(&self.db).await? {
// 			return Ok(GetUniCitySolvableProblem::CountryWrong(country))
// 		}
// 		let mut names = query!("SELECT nombre FROM Ciudad WHERE pais = ? AND region = ?", uni.pais, uni.region).fetch_all(&self.db).await?.into_iter().map(|x| x.nombre).collect::<Vec<_>>();
// 		let country_exists = query!("SELECT COUNT(*) as n FROM Pais WHERE codigo = ?", uni.pais).fetch_one(&self.db).await?.n != 0;
// 		let country = CountryId::new(uni.pais.to_string(), country_exists);
// 		let city = match names.len() {
// 			0 => CityId::new_inexistant(uni.region.to_string(), country),
// 			1 => CityId::new_single(uni.region.to_string(), names.remove(0), country),
// 			_ => CityId::new_multiple(uni.region.to_string(), names, country)
// 		};
//         Ok(GetUniCitySolvableProblem::UniDoesntExist { city, uni: uni.universidad })
//     }

//     pub async fn get_all_universities(&self) -> sqlx::Result<Vec<University>> {
//         query_as!(University, "SELECT nombre as name, numero as number, region, pais as country, ciudad as city, lat, lon FROM Universidad").fetch_all(&self.db).await
//     }
// }

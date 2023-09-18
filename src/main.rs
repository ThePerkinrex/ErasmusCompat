use std::{borrow::Cow, path::Path};

use csv::{ReaderBuilder, StringRecord};
use sqlx::{query, SqlitePool};

use database::Database;

mod database;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let info = Posicion {
        // pais: Some(0),
        // ciudad: Some(1),
        // universidad: Some(2),
        codigo_erasmus: 3,
        nivel_estudios: Some(5),
        plazas: Some(6),
        meses: Some(7),
        idioma: Some(8),
        observaciones: Some(9),
    };
    let db = SqlitePool::connect("sqlite:db.sqlite").await.unwrap();
    let db = Database::new(db);
    load_csv(&db, info, "Juan", "destinos/Erasmus - Destinos Juan.csv")
        .await
        .unwrap();
}

#[derive(Debug)]
struct Posicion {
    codigo_erasmus: usize,
    nivel_estudios: Option<usize>,
    plazas: Option<usize>,
    meses: Option<usize>,
    idioma: Option<usize>,
    observaciones: Option<usize>,
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

async fn load_csv<P: AsRef<Path> + Send>(
    db: &Database,
    info: Posicion,
    usuario: &str,
    path: P,
) -> csv::Result<()> {
    let mut reader = ReaderBuilder::new()
        .double_quote(true)
        .has_headers(true)
        .from_path(path)?;
    
    // add_persona(state, usuario).await.unwrap();
    for record in reader.records().skip(64) {
        let record = record?;
        let codigo_erasmus = record.get(info.codigo_erasmus).unwrap();
        let codigo_erasmus = ErasmusCode::try_from(codigo_erasmus).unwrap();
        db.get_uni_city(&codigo_erasmus).await.unwrap();
        // let n = query!("SELECT count(*) as n FROM Universidad WHERE numero = ? AND pais = ? AND region = ?", codigo_erasmus.universidad, codigo_erasmus.pais, codigo_erasmus.region).fetch_one(&state.pool).await.unwrap().n;
        // if n!=1 {
        //     println!("{codigo_erasmus} {n}");
        // }
        // let pais = get_elem(info.pais, &record);
        // let ciudad = get_elem(info.ciudad, &record);
        // let universidad = get_elem(info.universidad, &record);
        // add_pais(state, &codigo_erasmus.pais, pais.as_deref())
        //     .await
        //     .unwrap();
        // add_ciudad(
        //     state,
        //     &codigo_erasmus.ciudad,
        //     &codigo_erasmus.pais,
        //     ciudad.as_deref(),
        // )
        // .await
        // .unwrap();
        // add_universidad(
        //     state,
        //     codigo_erasmus.universidad,
        //     &codigo_erasmus.ciudad,
        //     &codigo_erasmus.pais,
        //     universidad.as_deref(),
        // )
        // .await
        // .unwrap();
        // add_destino(
        //     state,
        //     codigo_erasmus.universidad,
        //     &codigo_erasmus.ciudad,
        //     &codigo_erasmus.pais,
        //     usuario,
        // )
        // .await
        // .unwrap();
        // let info = InfoDestino {
        //     universidad: codigo_erasmus.universidad,
        //     ciudad: codigo_erasmus.ciudad,
        //     pais: codigo_erasmus.pais,
        //     persona: usuario.into(),
        //     nivel_estudios: get_elem(info.nivel_estudios, &record),
        //     plazas: get_elem(info.plazas, &record).and_then(|x| x.parse().ok()),
        //     meses: get_elem(info.meses, &record).and_then(|x| x.parse().ok()),
        //     idioma: get_elem(info.idioma, &record),
        //     observaciones: get_elem(info.observaciones, &record),
        // };
        // add_destino_info(state, info).await.unwrap();
        // println!();
    }
    Ok(())
}

fn get_elem(pos: Option<usize>, record: &StringRecord) -> Option<Cow<'_, str>> {
    pos.and_then(|x| record.get(x))
        .map(|x| x.replace("\r\n", " ").replace('\n', " ").into())
}

// async fn add_persona(state: &mut State, nombre: &str) -> sqlx::Result<()> {
//     let n = query!(
//         "SELECT count(*) as n FROM Persona WHERE persona = ?",
//         nombre
//     )
//     .fetch_one(&state.pool)
//     .await?
//     .n;
//     if n == 0 {
//         query!("INSERT INTO Persona(persona) VALUES(?)", nombre)
//             .execute(&state.pool)
//             .await?;
//     }
//     Ok(())
// }

// async fn add_destino(
//     state: &mut State,
//     universidad: u32,
//     ciudad: &str,
//     pais: &str,
//     persona: &str,
// ) -> sqlx::Result<()> {
//     let n = query!("SELECT count(*) as n FROM Destino WHERE universidad = ? AND pais = ? AND ciudad = ? AND persona = ?", universidad, pais, ciudad, persona).fetch_one(&state.pool).await?.n;
//     if n == 0 {
//         query!(
//             "INSERT INTO Destino(universidad, ciudad, pais, persona) VALUES(?, ?, ?, ?)",
//             universidad,
//             ciudad,
//             pais,
//             persona
//         )
//         .execute(&state.pool)
//         .await?;
//     }
//     Ok(())
// }

// struct InfoDestino<'a> {
//     universidad: u32,
//     ciudad: Cow<'a, str>,
//     pais: Cow<'a, str>,
//     persona: Cow<'a, str>,
//     nivel_estudios: Option<Cow<'a, str>>,
//     plazas: Option<u32>,
//     meses: Option<u32>,
//     idioma: Option<Cow<'a, str>>,
//     observaciones: Option<Cow<'a, str>>,
// }

// async fn add_destino_info(state: &mut State, info: InfoDestino<'_>) -> sqlx::Result<()> {
//     let idx = query!("SELECT count(*) as n FROM OpcionDestino WHERE universidad = ? AND pais = ? AND ciudad = ? AND persona = ?", info.universidad, info.pais, info.ciudad, info.persona).fetch_one(&state.pool).await?.n;

//     let exists = if idx != 0 {
//         query!(
//             "SELECT plazas,nivel_estudios,meses,idioma,observaciones FROM OpcionDestino WHERE universidad = ? AND pais = ? AND ciudad = ? AND persona = ?",
//             info.universidad,
//             info.pais,
//             info.ciudad,
//             info.persona)
//         .fetch_all(&state.pool).await?
//         .into_iter()
//         .any(|record|
//             record.nivel_estudios.as_deref() == info.nivel_estudios.as_deref()
//             && record.plazas == info.plazas.map(|x| x as i64)
//             && record.meses == info.meses.map(|x| x as i64)
//             && record.idioma.as_deref() == info.idioma.as_deref()
//             && record.observaciones.as_deref() == info.observaciones.as_deref()
//         )
//     } else {
//         false
//     };
//     // println!("Existe: {exists}");
//     if !exists {
//         // println!("opcion: {idx}");
//         query!(
//             "INSERT INTO OpcionDestino(opcion, universidad, ciudad, pais, persona, plazas, nivel_estudios, meses, idioma, observaciones) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
//             idx,
//             info.universidad,
//             info.ciudad,
//             info.pais,
//             info.persona,
//             info.plazas,
//             info.nivel_estudios,
//             info.meses,
//             info.idioma,
//             info.observaciones
//         )
//         .execute(&state.pool)
//         .await?;
//     }
//     Ok(())
// }

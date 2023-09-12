use std::{path::Path, borrow::Cow};

use csv::ReaderBuilder;

fn main() {
    println!("Hello, world!");
    let info = Posicion {
        pais: Some(0),
        ciudad: Some(1),
        universidad: Some(2),
        codigo_erasmus: 3,
        nivel_estudios: Some(5),
        plazas: Some(6),
        meses: Some(7),
        idioma: Some(8),
        observaciones: Some(9),
    };

    load_csv(info, "Erasmus - Destinos Juan(1).csv").unwrap();
}

#[derive(Debug)]
struct Posicion {
    pais: Option<usize>,
    ciudad: Option<usize>,
    universidad: Option<usize>,
    codigo_erasmus: usize,
    nivel_estudios: Option<usize>,
    plazas: Option<usize>,
    meses: Option<usize>,
    idioma: Option<usize>,
    observaciones: Option<usize>,
}

#[derive(Debug)]
struct CodigoErasmus<'a> {
    pais: Cow<'a, str>,
    ciudad: Cow<'a, str>,
    universidad: usize
}

impl std::fmt::Display for CodigoErasmus<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{:02}", self.pais, self.ciudad, self.universidad)
    }
}

impl<'a> TryFrom<&'a str> for CodigoErasmus<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (pais, ciu_uni) = value.split_once(' ').ok_or(())?;
        let i = ciu_uni.char_indices().find(|(_, c)| c.is_numeric()).ok_or(())?.0;
        let ciudad = ciu_uni[..i].trim();
        let uni = ciu_uni[i..].trim().parse().map_err(|_| ())?;
        Ok(Self {
            pais: pais.into(), ciudad: ciudad.into(), universidad: uni
        })
    }
}

fn load_csv<P: AsRef<Path>>(info: Posicion, path: P) -> csv::Result<()> {
    let mut reader = ReaderBuilder::new().double_quote(true).has_headers(true).from_path(path)?;
    for record in reader.records() {
        let record = record?;
        let codigo_erasmus = record.get(info.codigo_erasmus).unwrap();
        let codigo_erasmus = CodigoErasmus::try_from(codigo_erasmus).unwrap();
        print!("{codigo_erasmus} ");
        if let Some(pais) = info.pais {
            let pais = record.get(pais).unwrap().replace("\r\n", " ").replace('\n', " ");
            print!("{pais} ");
        }
        if let Some(ciudad) = info.ciudad {
            let ciudad = record.get(ciudad).unwrap().replace("\r\n", " ").replace('\n', " ");
            print!("{ciudad} ");
        }
        println!();
    }
    Ok(())
}
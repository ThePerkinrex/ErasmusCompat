#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UniCity {
    pub uni: Option<String>,
    pub city: String,
}

#[derive(Debug,  serde::Serialize)]
pub struct Country {
    pub erasmus_code: String,
    pub name: String,
    pub iso_code: String,
}

#[derive(Debug)]
pub struct City {
    pub region: String,
    pub name: String,
    pub coords: Option<(f64, f64)>,
}

#[derive(Debug)]
pub struct Uni {
    pub code: i32,
    pub name: String,
    pub coords: Option<(f64, f64)>,
}

#[derive(Debug, serde::Serialize)]
pub struct CountryId {
    code: String,
    exists: bool,
}

impl CountryId {
    pub const fn new(code: String, exists: bool) -> Self {
        Self { code, exists }
    }

    pub const fn exists(&self) -> bool {
        self.exists
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CityId {
    region: String,
    data: CityData,
    country: CountryId,
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
enum CityData {
    Single { name: String },
    Multiple { names: Vec<String> },
    None,
}

impl CityData {
    fn iter(&self) -> CityNameIter {
        match self {
            Self::Single { name } => CityNameIter::Single(std::iter::once(&name)),
            Self::Multiple { names } => CityNameIter::Multiple(names.iter().map(String::as_str)),
            Self::None => CityNameIter::None,
        }
    }
}

impl CityId {
    pub const fn new_inexistant(region: String, country: CountryId) -> Self {
        Self {
            region,
            data: CityData::None,
            country,
        }
    }

    pub const fn new_single(region: String, name: String, country: CountryId) -> Self {
        Self {
            region,
            data: CityData::Single { name },
            country,
        }
    }

    pub const fn new_multiple(region: String, names: Vec<String>, country: CountryId) -> Self {
        Self {
            region,
            data: CityData::Multiple { names },
            country,
        }
    }

    pub const fn exists(&self) -> bool {
        !matches!(self.data, CityData::None) && self.country.exists()
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn name(&self) -> CityNameIter {
        self.data.iter()
    }

    pub fn country_code(&self) -> &str {
        self.country.code()
    }
}

pub enum CityNameIter<'a> {
    Single(std::iter::Once<&'a str>),
    Multiple(std::iter::Map<std::slice::Iter<'a, String>, fn(&String) -> &str>),
    None,
}

impl<'a> Iterator for CityNameIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CityNameIter::Single(x) => x.next(),
            CityNameIter::Multiple(x) => x.next(),
            CityNameIter::None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::None => (0, Some(0)),
            x => x.size_hint(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct University {
    pub number: i64,
    pub country: String,
    pub region: String,
    pub city: String,
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

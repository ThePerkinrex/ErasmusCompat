use std::time::Duration;

use tokio::time::Instant;

use reqwest::Client;
use url::Url;

#[derive(serde::Deserialize, Debug)]
pub struct Place {
    #[serde(deserialize_with = "de_coord")]
    pub lat: f64,
    #[serde(deserialize_with = "de_coord")]
    pub lon: f64,
}

#[derive(Debug)]
struct F64Visitor;

impl<'de> serde::de::Visitor<'de> for F64Visitor {
    type Value = f64;

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(|e| {
            E::invalid_value(
                serde::de::Unexpected::Str(v),
                &format!("Error parsing float string: {e}").as_str(),
            )
        })
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "A floating point number or a string with a floating point number"
        )
    }
}

fn de_coord<'de, D>(de: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    de.deserialize_any(F64Visitor)
}

pub struct PlaceLocator {
    next_possible_req: Instant,
    client: Client,
}

impl PlaceLocator {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .user_agent(concat!("erasmus_validator", "/", env!("CARGO_PKG_VERSION"),))
            .build()?;
        Ok(Self {
            next_possible_req: Instant::now(),
            client,
        })
    }

    pub async fn get_place(&mut self, location: &str) -> Option<Place> {
		if self.next_possible_req.elapsed().is_zero() {
			println!("Waiting for timeout in place search")
		}
        tokio::time::sleep_until(self.next_possible_req).await;
		println!("Searching for {location:?}");
        let res = self
            .client
            .get(
                Url::parse_with_params(
                    "https://nominatim.openstreetmap.org/search",
                    [("q", location), ("format", "json")],
                )
                .unwrap(),
            )
            .send()
            .await
            .ok()?;
		
		self.next_possible_req = Instant::now() + Duration::from_secs(1);
        res.json::<Vec<Place>>().await.unwrap().into_iter().next()
    }
}

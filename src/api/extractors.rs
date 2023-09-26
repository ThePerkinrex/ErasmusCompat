use axum::{
    extract::FromRequest,
    http::{header, HeaderValue, Request},
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;

use crate::database::model::UniCity;

#[derive(Debug, serde::Deserialize)]
pub struct DestinationsFix {
    pub record_number: usize,
    #[serde(flatten)]
    pub fix: DestinationsFixKind
}

#[derive(Debug, serde::Deserialize)]
pub struct Country {
    pub name: String,
    pub iso_code: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub long: f64
}


#[derive(Debug, serde::Deserialize)]
pub struct Uni {
    pub name: String,
    pub lat: f64,
    pub long: f64,
    pub street: String,
    pub postal_code: String,
    pub website: String
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum DestinationsFixKind {
    SelectOption(UniCity),
    UpdateCountry {
        name: String
    },
    AddUni {
        country: Option<Country>,
        city: Option<City>,
        uni: Uni
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DestinationsData {
    pub csv: String,
	#[serde(default)]
    pub fixes: Vec<DestinationsFix>,
}

#[axum::async_trait]
impl<S, B> FromRequest<S, B> for DestinationsData
where
    String: FromRequest<S, B>,
    Json<Self>: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(header::CONTENT_TYPE)
            .map(HeaderValue::to_str);

        match content_type {
            Some(Ok("text/csv")) => Ok(Self {
                csv: String::from_request(req, state)
                    .await
                    .map_err(IntoResponse::into_response)?,
                fixes: Vec::new(),
            }),
            Some(Ok("application/json")) => Json::<Self>::from_request(req, state)
                .await
                .map_err(IntoResponse::into_response)
                .map(|Json(x)| x),
            Some(Ok(x)) => Err((
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("Only accepting text/csv or application/json content types, found: {x}"),
            )
                .into_response()),
            Some(Err(e)) => Err((
                StatusCode::BAD_REQUEST,
                format!("Error decoding content-type: {e}"),
            )
                .into_response()),
            None => Err((
                StatusCode::BAD_REQUEST,
                "No content-type provided, accepting only text/csv or application/json",
            )
                .into_response()),
        }
    }
}

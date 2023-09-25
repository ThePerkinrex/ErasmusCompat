use axum::{
    extract::FromRequest,
    http::{header, HeaderValue, Request},
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;

#[derive(Debug, serde::Deserialize)]
pub struct DestinationsData {
    csv: String,
	#[serde(default)]
    fixes: Vec<()>,
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

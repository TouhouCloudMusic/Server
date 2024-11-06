use crate::AppState;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{debug_handler, Json, Router};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::json;
use xxhash_rust::xxh3::xxh3_64;

pub fn router() -> Router<AppState> {
    Router::new().route("avatar", post(upload_avatar))
}

#[debug_handler]
async fn upload_avatar(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let name = field.name().unwrap_or("").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();

        if name != "data" {
            return Err(StatusCode::BAD_REQUEST);
        }

        if !content_type.starts_with("image/") {
            return Err(StatusCode::BAD_REQUEST);
        }

        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        let data_hash = xxh3_64(&data);

        let filename = URL_SAFE.encode(data_hash.to_be_bytes());

        // TODO

        return Ok(Json(json!({
            "message": "Ok"
        })));
    }

    Ok(Json(json!({
        "message": "Err"
    })))
}

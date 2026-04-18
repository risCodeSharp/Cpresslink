use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing,
};

use crate::{
    config::state::AppState, services::redirect_service::RedirectService, utils::{response::ApiResponse}
};

pub async fn redirect(
    State(state): State<AppState>,
    Path((username, slug)): Path<(String, String)>

) -> impl IntoResponse {
    println!("{slug}");
    match RedirectService::get_link(state, &username, &slug).await {
        Ok(url) => Redirect::to(&url.destination).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(&e.to_string(), StatusCode::NOT_FOUND)),
        ).into_response(),
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/r/{username}/{slug}", routing::get(redirect))
}

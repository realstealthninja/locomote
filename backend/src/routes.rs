use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::user, handlers::card};


pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/v1/users", user_router(state.clone()))
        .with_state(state)
}

pub fn user_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(user::post_user))
        .with_state(state)
}

pub fn card_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(card::post_card))
        .with_state(state)
}

async fn root() -> &'static str {
    "Ok"
}
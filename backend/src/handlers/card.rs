use axum::{
    Json,
    extract::{State},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::state::AppState;

#[derive(Deserialize, Debug)]
pub struct CardRequest {
    pub code: String,
    pub user_id: i32,
}

#[derive(Serialize, FromRow, Debug)]
pub struct CardResponse {
    pub id: i32,
    pub user_id: i32,
}

pub async fn post_card(
    State(state): State<AppState>,
    Json(card): Json<CardRequest>,
) -> Json<CardResponse> {
    Json(
        sqlx::query_as::<_, CardResponse>(
            "INSERT INTO card (code, user_id) VALUES ($1, $2) RETURNING id, user_id",
        )
        .bind(card.code)
        .bind(card.user_id)
        .fetch_one(&state.pool)
        .await
        .expect("unable to insert card into database"),
    )
}
    

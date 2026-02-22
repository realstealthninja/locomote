use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct VerificationRequest {
    card_id: i32,
    scanner_id: i32,
}

#[derive(Serialize)]
pub struct VerificationResponse {
    success: bool,
    error: String
}


pub async fn post_verify(
    State(state): State<AppState>,
    Json(req): Json<VerificationRequest>,
) -> Json<VerificationResponse> {
    let card = sqlx::query("SELECT id FROM card WHERE card_id = $1").bind(req.card_id).fetch_one(&state.pool).await;

    if card.is_err() {
        return Json(
            VerificationResponse {
              success:false,
              error: "No such card".to_owned()
            }
        )
    } 
    let card_id: i32 = card.unwrap().get("id");
    let ticket = sqlx::query("`
        SELECT * FROM ticket WHERE card_id = $1, scanner_id = $2 ").bind(card_id).bind(req.scanner_id).fetch_optional(&state.pool).await;

    if ticket.is_err() {
        return Json(
            VerificationResponse { success: true, error: "No such ticket".to_owned() }
        )

    }
    return Json(
        VerificationResponse { success: true, error: "No error".to_owned() }
    )
    
}
use axum::extract::{self, State};
use serde::Serialize;

use crate::{models::card::{NewCard, create_card}, state::AppState};

#[derive(Serialize)]
pub struct CardResponse {
    pub id: i32,
    pub card_id: String,
    pub user_id: i32,
    pub balance: f32
}


pub async fn post_card(
    State(state): State<AppState>,
    extract::Json(new_card): extract::Json<NewCard>,
) ->  extract::Json<CardResponse> {
    let card = create_card(&state.pool, new_card).await;

    extract::Json(CardResponse{
        id: card.id,
        card_id: card.card_id,
        user_id: card.user_id,
        balance: card.balance
    })
}

pub async fn get_card(
    State(state): State<AppState>
) {

}
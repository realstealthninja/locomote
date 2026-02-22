use crate::{handlers::{card::CardResponse, user}, state::AppState};
use axum::{
    Json,
    extract::{self, State},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserResponse {
    id: i32,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct UserPost {
    username: String,
    password: String,
    email: String,
}


#[derive(Debug, Serialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    cards: Vec<CardResponse>
    
}

pub async fn post_user(
    State(state): State<AppState>,
    Json(userreq): Json<UserPost>,
) -> Json<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        "INSERT INTO usr (username, email, password) VALUES ($1, $2, $3) RETURNING id, username",
    )
    .bind(userreq.username)
    .bind(userreq.email)
    .bind(userreq.password)
    .fetch_one(&state.pool)
    .await
    .expect("Could not insert into database");

    Json(user)
}






pub async fn delete_user() {}

pub async fn update_user() {}

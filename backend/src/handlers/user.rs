use crate::models::user::{NewUser, create_user};
use crate::state::AppState;
use axum::{Json, extract::{self, State}};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    id: i32,
    username: String,
}


pub async fn post_user(
    State(state): State<AppState>,
    extract::Json(new_user): extract::Json<NewUser>,
) -> extract::Json<UserResponse>
{
    println!("POST: USER");
    let user = create_user(&state.pool, new_user).await;

    Json(UserResponse {
        id: user.id,
        username: user.username
    })
}

pub async fn delete_user() {}

pub async fn update_user() {}

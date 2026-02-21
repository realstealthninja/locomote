mod routes;
mod handlers;
mod state;

use std::env;
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};

use crate::{routes::router, state::AppState};


#[tokio::main]  
async fn main() -> Result<(), std::io::Error>{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
                            .expect("Database url in env must be set");

    let pool = PgPoolOptions::new().connect(&db_url).await
                            .expect("Couldnt connect to db");

    sqlx::migrate!().run(&pool).await
                            .expect("Couldnt migrate database");
                        
    let app_state = AppState{pool};
    let router = router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0:8080").await.unwrap();
    
    println!("Lisenting on port 8080");
    axum::serve(listener, router).await
}


mod schema;
mod models;
mod routes;
mod state;
mod handlers;

use std::env;
use deadpool_diesel::{Manager, Pool, Runtime};
use dotenvy::dotenv;

use crate::{routes::router, state::AppState};


#[tokio::main]  
async fn main() -> Result<(), std::io::Error>{
    tracing_subscriber::fmt::init();
    
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("Database url in env must be set");
    let manager = Manager::new(url.to_string(), Runtime::Tokio1);

    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create pool");

    let app_state = AppState{ pool };
    let router = router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0:8080").await.unwrap();
    
    println!("Lisenting on port 8080");
    axum::serve(listener, router).await
}


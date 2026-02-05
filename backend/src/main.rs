mod schema;
mod models;

use std::env;

use diesel::prelude::*;
use axum::{
    routing::{get},
    Router
};

use dotenvy::dotenv;

fn db_connect() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("Database url in env must be set");
    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {url}"))
}

#[tokio::main]  
async fn main() -> Result<(), std::io::Error>{
    tracing_subscriber::fmt::init();
    
    db_connect();

    let app = Router::new()
     .route("/", get(root));


    

    let listener = tokio::net::TcpListener::bind("0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await
}


async fn root() -> &'static str {
    "Hello world"
}
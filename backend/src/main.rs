use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

fn db_connect() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("Database url in env must be set");
    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {url}"))
}


fn main(){
    println!("hello, world!");
    db_connect();
}   

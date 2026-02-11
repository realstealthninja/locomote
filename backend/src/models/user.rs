use diesel::{
    RunQueryDsl, Selectable, SelectableHelper,
    prelude::{Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::schema::user;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
}

pub async fn create_user(pool: &deadpool_diesel::postgres::Pool, user: NewUser) -> User {
    

    let conn = pool.get().await.expect("Cant get connection to pool");

    conn.interact(|conn| {
        let new_user = user;
        diesel::insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
        }
    ).await.expect("User couldnt be inserted")
}

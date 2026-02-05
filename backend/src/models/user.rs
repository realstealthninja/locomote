use diesel::{Selectable, prelude::Queryable};


#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::user)]
pub struct User {
    id: i32,
    user_id: i32,
    username: String,
    creation: String
}
use diesel::{Selectable, prelude::Queryable};



#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::useraccount)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    id: i32,
    user_id: i32,
    username: String,
    password_hash: String
}
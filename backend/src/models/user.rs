use diesel::{Selectable, prelude::Queryable};

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    user_id: String,
    username: String,
    creation: i32,
}

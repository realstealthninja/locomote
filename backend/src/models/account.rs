use diesel::{Selectable, prelude::Queryable};



#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::useraccount)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub password_hash: String
}
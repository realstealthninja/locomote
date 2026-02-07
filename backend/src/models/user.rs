use diesel::{Selectable, prelude::Queryable};
use time::PrimitiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub username: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at:PrimitiveDateTime
}

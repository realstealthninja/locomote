use diesel::{Selectable, prelude::Queryable};
use time::PrimitiveDateTime;


#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::scanner)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Scanner {
    pub id: i32,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>
}


use diesel::{Selectable, prelude::Queryable};


#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::scanner)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Scanner {
    pub id: i32,
    pub scanner_id: i32
}
use diesel::{Selectable, prelude::Queryable};


#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::scanner)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Scanner {
    id: i32,
    scanner_id: i32
}
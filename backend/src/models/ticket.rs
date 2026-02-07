use diesel::{Selectable, prelude::Queryable};
use time::{Date, PrimitiveDateTime, Time};


#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::ticket)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Ticket {
    pub id: i32,
    pub card_id: i32,
    pub user_id: i32,
    pub origin: String,
    pub destination: String,
    pub validity: PrimitiveDateTime,
    pub travelling_at: PrimitiveDateTime,
    pub cost: f32
}
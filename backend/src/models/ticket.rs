use diesel::{Selectable, prelude::{Associations, Identifiable, Queryable}};
use time::{Date, PrimitiveDateTime, Time};

use crate::models::{card::Card, user::User};


#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name= crate::schema::ticket)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Card))]
pub struct Ticket {
    pub id: i32,
    pub card_id: i32,
    pub user_id: i32,
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub validity: Option<PrimitiveDateTime>,
    pub travelling_at: Option<PrimitiveDateTime>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
    pub cost: f32
}
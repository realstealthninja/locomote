use diesel::{Selectable, prelude::Queryable,};

#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::card)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card {
    pub id: i32,
    pub card_id: i32,
    pub user_id: i32,
    pub balance: f32
}
use diesel::{RunQueryDsl, Selectable, SelectableHelper, prelude::{Associations, Identifiable, Insertable, Queryable}};
use serde::Deserialize;
use time::PrimitiveDateTime;

use crate::{models::user::User, schema::card};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name= card)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card {
    pub id: i32,
    pub user_id: i32,
    pub disabled: bool,
    pub card_id: String,
    pub balance: f32,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = card)]
pub struct NewCard {
    pub user_id: i32,
    pub card_id: String,
}

pub async fn create_card(pool: &deadpool_diesel::postgres::Pool, card: NewCard) -> Card {
    let conn = pool.get().await.expect("Cant get connection to pool");

    conn.interact(|conn| {
        let new_card = card;
        diesel::insert_into(card::table)
        .values(&new_card)
        .returning(Card::as_returning())
        .get_result(conn)
        .expect("Card couldnt be inserted")
        }
    ).await.expect("Card couldnt be inserted")
}

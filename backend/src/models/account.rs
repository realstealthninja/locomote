use diesel::{Selectable, prelude::{Associations, Identifiable, Queryable}};

use crate::models::user::User;



#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name=crate::schema::useraccount)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub password_hash: String
}
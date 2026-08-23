use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{models::enums::UserRole, schema::users};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,

    pub email: String,
    pub password: String,
    pub username: String,

    pub role: UserRole,

    pub is_super: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,

    pub role: UserRole,

    pub is_super: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,

    pub role: Option<UserRole>,

    pub is_super: Option<bool>,
}

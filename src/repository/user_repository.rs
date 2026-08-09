use std::collections::HashMap;

use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::{
    models::{
        enums::UserRole,
        user::{NewUser, UpdateUser, User},
    },
    schema::users,
};

pub struct UserRepository;

impl UserRepository {
    pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<User>> {
        let mut query = users::table
            .select(User::as_select())
            .order(users::id.asc())
            .into_boxed();

        if let Some(user_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(users::id.eq(user_id));
        }
        if let Some(email) = params
            .get("email")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::email.eq(email));
        }
        if let Some(name) = params
            .get("name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::name.ilike(format!("%{}%", name)));
        }
        if let Some(role) = params
            .get("role")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::role.eq(UserRole::from(role)));
        }

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, user_id: i64) -> QueryResult<User> {
        users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .first(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(user_email))
            .select(User::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: i64,
        update_user: &UpdateUser,
    ) -> QueryResult<User> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(update_user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i64) -> QueryResult<usize> {
        diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)
    }
}

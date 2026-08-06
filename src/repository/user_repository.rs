use std::collections::HashMap;

use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::{
    models::user::{NewUser, UpdateUser, User},
    schema::users,
    schema::users::dsl::*,
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
        let mut query = users.select(User::as_select()).order(id.asc()).into_boxed();

        if let Some(user_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(id.eq(user_id));
        }

        if let Some(user_email) = params
            .get("email")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(email.eq(user_email));
        }

        if let Some(user_name) = params
            .get("name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(name.ilike(format!("%{}%", user_name)));
        }

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, user_id: i64) -> QueryResult<User> {
        users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        users
            .filter(email.eq(user_email))
            .select(User::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: i64,
        update_user: &UpdateUser,
    ) -> QueryResult<User> {
        diesel::update(users.filter(id.eq(user_id)))
            .set(update_user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i64) -> QueryResult<usize> {
        diesel::delete(users.filter(id.eq(user_id))).execute(conn)
    }
}

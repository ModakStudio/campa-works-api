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

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users.select(User::as_select()).order(id.asc()).load(conn)
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

use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        professor::{NewProfessor, Professor, UpdateProfessor},
        user::User,
    },
    schema::{professor, users},
};

pub struct ProfessorRepository;

impl ProfessorRepository {
    pub fn create(conn: &mut PgConnection, new_professor: &NewProfessor) -> QueryResult<Professor> {
        diesel::insert_into(professor::table)
            .values(new_professor)
            .returning(Professor::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(Professor, User)>> {
        let mut query = professor::table
            .inner_join(users::table)
            .select((Professor::as_select(), User::as_select()))
            .into_boxed();

        if let Some(professor_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(professor::id.eq(professor_id));
        }

        if let Some(user_name) = params
            .get("user_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::name.ilike(format!("%{}%", user_name)));
        }

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> QueryResult<(Professor, User)> {
        professor::table
            .inner_join(users::table)
            .filter(professor::id.eq(professor_id))
            .select((Professor::as_select(), User::as_select()))
            .first(conn)
    }

    pub fn find_by_user_id(
        conn: &mut PgConnection,
        target_user_id: i64,
    ) -> QueryResult<(Professor, User)> {
        professor::table
            .inner_join(users::table)
            .filter(professor::user_id.eq(target_user_id))
            .select((Professor::as_select(), User::as_select()))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_id: i64,
        update_professor: &UpdateProfessor,
    ) -> QueryResult<Professor> {
        diesel::update(professor::table.filter(professor::id.eq(professor_id)))
            .set(update_professor)
            .returning(Professor::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, professor_id: i64) -> QueryResult<usize> {
        diesel::delete(professor::table.filter(professor::id.eq(professor_id))).execute(conn)
    }
}

use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        enums::{ProfessorPosition, ProfessorStatus},
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

        if let Some(id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(professor::id.eq(id));
        }
        if let Some(position) = params
            .get("position")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::position.eq(ProfessorPosition::from(position)));
        }
        if let Some(office) = params
            .get("office")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::office.ilike(format!("%{}%", office)));
        }
        if let Some(tel) = params
            .get("tel")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::tel.ilike(format!("%{}%", tel)));
        }
        if let Some(research_field) = params
            .get("research_field")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::research_field.ilike(format!("%{}%", research_field)));
        }
        if let Some(status) = params
            .get("status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::status.eq(ProfessorStatus::from(status)));
        }

        if let Some(user_id) = params
            .get("user_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor::user_id.eq(user_id));
        }
        if let Some(user_email) = params
            .get("user_email")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::email.eq(user_email));
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

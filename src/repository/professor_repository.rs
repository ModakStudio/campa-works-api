use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        enums::*,
        professor::{NewProfessor, Professor, UpdateProfessor},
        semester::Semester,
        user::User,
    },
    schema::{professor, semester, users},
};

#[macro_export]
macro_rules! apply_professor_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(professor_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor::id.eq(professor_id));
        }

        query = crate::apply_user_query_filters!(query, $params);

        if let Some(position) = $params
            .get("position")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::position.eq(ProfessorPosition::from(position)));
        }
        if let Some(office) = $params
            .get("office")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::office.ilike(format!("%{}%", office)));
        }
        if let Some(tel) = $params
            .get("tel")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::tel.ilike(format!("%{}%", tel)));
        }
        if let Some(research_field) = $params
            .get("research_field")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::research_field.ilike(format!("%{}%", research_field)));
        }

        if let Some(appointed_at) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(semester::id.eq(appointed_at));
        }

        if let Some(professor_status) = $params
            .get("professor_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(professor::professor_status.eq(ProfessorStatus::from(professor_status)));
        }

        query
    }};
}

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
    ) -> QueryResult<Vec<(Professor, User, Semester)>> {
        let mut query = professor::table
            .inner_join(users::table)
            .inner_join(semester::table)
            .select((
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .into_boxed();

        query = apply_professor_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> QueryResult<(Professor, User, Semester)> {
        professor::table
            .inner_join(users::table)
            .inner_join(semester::table)
            .filter(professor::id.eq(professor_id))
            .select((
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_user_id(
        conn: &mut PgConnection,
        target_user_id: i64,
    ) -> QueryResult<(Professor, User, Semester)> {
        professor::table
            .inner_join(users::table)
            .inner_join(semester::table)
            .filter(professor::user_id.eq(target_user_id))
            .select((
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
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

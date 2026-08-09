use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        enums::{ProfessorPosition, ProfessorStatus, SemesterStatus, SemesterType},
        professor::Professor,
        professor_credit::{NewProfessorCredit, ProfessorCredit, UpdateProfessorCredit},
        semester::Semester,
        user::User,
    },
    schema::{professor, professor_credit, semester, users},
};

pub struct ProfessorCreditRepository;

impl ProfessorCreditRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_professor_credit: &NewProfessorCredit,
    ) -> QueryResult<ProfessorCredit> {
        diesel::insert_into(professor_credit::table)
            .values(new_professor_credit)
            .returning(ProfessorCredit::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(ProfessorCredit, Professor, User, Semester)>> {
        let mut query = professor_credit::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .into_boxed();

        if let Some(professor_credit_id) =
            params.get("id").and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor_credit::id.eq(professor_credit_id));
        }

        if let Some(professor_id) = params
            .get("professor_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor_credit::professor_id.eq(professor_id));
        }
        if let Some(professor_position) = params
            .get("professor_position")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(professor::position.eq(ProfessorPosition::from(professor_position)));
        }
        if let Some(professor_office) = params
            .get("professor_office")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::office.ilike(format!("%{}%", professor_office)));
        }
        if let Some(professor_tel) = params
            .get("professor_tel")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::tel.ilike(format!("%{}%", professor_tel)));
        }
        if let Some(professor_research_field) = params
            .get("professor_research_field")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(professor::research_field.ilike(format!("%{}%", professor_research_field)));
        }
        if let Some(professor_status) = params
            .get("professor_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::status.eq(ProfessorStatus::from(professor_status)));
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

        if let Some(semester_id) = params
            .get("semester_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor_credit::semester_id.eq(semester_id));
        }
        if let Some(semester_year) = params
            .get("semester_year")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(semester::year.eq(semester_year));
        }
        if let Some(semester_semester_) = params
            .get("semester_semester_")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::semester_.eq(SemesterType::from(semester_semester_)));
        }
        if let Some(semester_status) = params
            .get("semester_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::status.eq(SemesterStatus::from(semester_status)));
        }

        if let Some(target_credit) = params
            .get("target_credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(professor_credit::target_credit.eq(target_credit));
        }

        query
            .select((
                ProfessorCredit::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        professor_credit_id: i64,
    ) -> QueryResult<(ProfessorCredit, Professor, User, Semester)> {
        professor_credit::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .filter(professor_credit::id.eq(professor_credit_id))
            .select((
                ProfessorCredit::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_credit_id: i64,
        update_professor_credit: &UpdateProfessorCredit,
    ) -> QueryResult<ProfessorCredit> {
        diesel::update(professor_credit::table.filter(professor_credit::id.eq(professor_credit_id)))
            .set(update_professor_credit)
            .returning(ProfessorCredit::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, professor_credit_id: i64) -> QueryResult<usize> {
        diesel::delete(professor_credit::table.filter(professor_credit::id.eq(professor_credit_id)))
            .execute(conn)
    }
}

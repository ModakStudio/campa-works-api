use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
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

        if let Some(semester_id) = params
            .get("semester_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor_credit::semester_id.eq(semester_id));
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

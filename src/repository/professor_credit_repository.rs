use diesel::prelude::*;

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
    ) -> QueryResult<Vec<(ProfessorCredit, Professor, User, Semester)>> {
        professor_credit::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
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

    pub fn find_by_professor_id_and_semester_id(
        conn: &mut PgConnection,
        professor_id: i64,
        semester_id: i64,
    ) -> QueryResult<(ProfessorCredit, Professor, User, Semester)> {
        professor_credit::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .filter(professor_credit::professor_id.eq(professor_id))
            .filter(professor_credit::semester_id.eq(semester_id))
            .select((
                ProfessorCredit::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_professor_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> QueryResult<Vec<(ProfessorCredit, Professor, User, Semester)>> {
        professor_credit::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .filter(professor_credit::professor_id.eq(professor_id))
            .select((
                ProfessorCredit::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_semester_id(
        conn: &mut PgConnection,
        semester_id: i64,
    ) -> QueryResult<Vec<(ProfessorCredit, Professor, User, Semester)>> {
        professor_credit::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .filter(professor_credit::semester_id.eq(semester_id))
            .select((
                ProfessorCredit::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .load(conn)
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

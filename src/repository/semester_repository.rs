use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::{
    models::semester::{NewSemester, Semester, UpdateSemester},
    schema::semester::dsl::*,
};

pub struct SemesterRepository;

impl SemesterRepository {
    pub fn create(conn: &mut PgConnection, new_semester: &NewSemester) -> QueryResult<Semester> {
        diesel::insert_into(semester)
            .values(new_semester)
            .returning(Semester::as_returning())
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Semester>> {
        semester
            .select(Semester::as_select())
            .order((year.desc(), semester_.asc()))
            .load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, semester_id: i64) -> QueryResult<Semester> {
        semester
            .filter(id.eq(semester_id))
            .select(Semester::as_select())
            .first(conn)
    }

    pub fn find_by_year_and_semester(
        conn: &mut PgConnection,
        target_year: i32,
        target_semester: crate::models::enums::SemesterType,
    ) -> QueryResult<Semester> {
        semester
            .filter(year.eq(target_year))
            .filter(semester_.eq(target_semester))
            .select(Semester::as_select())
            .into_boxed()
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        semester_id: i64,
        update_semester: &UpdateSemester,
    ) -> QueryResult<Semester> {
        diesel::update(semester.filter(id.eq(semester_id)))
            .set(update_semester)
            .returning(Semester::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, semester_id: i64) -> QueryResult<usize> {
        diesel::delete(semester.filter(id.eq(semester_id))).execute(conn)
    }
}

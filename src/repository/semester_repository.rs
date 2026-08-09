use std::collections::HashMap;

use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::{
    models::{
        enums::{SemesterStatus, SemesterType},
        semester::{NewSemester, Semester, UpdateSemester},
    },
    schema::semester,
};

pub struct SemesterRepository;

impl SemesterRepository {
    pub fn create(conn: &mut PgConnection, new_semester: &NewSemester) -> QueryResult<Semester> {
        diesel::insert_into(semester::table)
            .values(new_semester)
            .returning(Semester::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<Semester>> {
        let mut query = semester::table
            .select(Semester::as_select())
            .order((semester::year.desc(), semester::semester_.asc()))
            .into_boxed();

        if let Some(id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(semester::id.eq(id));
        }
        if let Some(year) = params
            .get("year")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(semester::year.eq(year));
        }
        if let Some(semester_) = params
            .get("semester_")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::semester_.eq(SemesterType::from(semester_)));
        }
        if let Some(status) = params
            .get("status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::status.eq(SemesterStatus::from(status)));
        }

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, semester_id: i64) -> QueryResult<Semester> {
        semester::table
            .filter(semester::id.eq(semester_id))
            .select(Semester::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        semester_id: i64,
        update_semester: &UpdateSemester,
    ) -> QueryResult<Semester> {
        diesel::update(semester::table.filter(semester::id.eq(semester_id)))
            .set(update_semester)
            .returning(Semester::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, semester_id: i64) -> QueryResult<usize> {
        diesel::delete(semester::table.filter(semester::id.eq(semester_id))).execute(conn)
    }
}

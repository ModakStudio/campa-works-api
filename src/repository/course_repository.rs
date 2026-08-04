use diesel::prelude::*;

use crate::{
    models::{
        course::{Course, NewCourse, UpdateCourse},
        major::Major,
        master_course::MasterCourse,
        semester::Semester,
    },
    schema::{course, major, master_course, semester},
};

pub struct CourseRepository;

impl CourseRepository {
    pub fn create(conn: &mut PgConnection, new_course: &NewCourse) -> QueryResult<Course> {
        diesel::insert_into(course::table)
            .values(new_course)
            .returning(Course::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(Course, MasterCourse, Semester, Major)>> {
        course::table
            .inner_join(master_course::table)
            .inner_join(semester::table)
            .inner_join(major::table)
            .select((
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_id: i64,
    ) -> QueryResult<(Course, MasterCourse, Semester, Major)> {
        course::table
            .inner_join(master_course::table)
            .inner_join(semester::table)
            .inner_join(major::table)
            .filter(course::id.eq(course_id))
            .select((
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        update_course: &UpdateCourse,
    ) -> QueryResult<Course> {
        diesel::update(course::table.filter(course::id.eq(course_id)))
            .set(update_course)
            .returning(Course::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> QueryResult<usize> {
        diesel::delete(course::table.filter(course::id.eq(course_id))).execute(conn)
    }
}

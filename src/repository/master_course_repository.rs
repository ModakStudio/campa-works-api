use diesel::prelude::*;

use crate::{
    models::master_course::{MasterCourse, NewMasterCourse, UpdateMasterCourse},
    schema::master_course::dsl::*,
};

pub struct MasterCourseRepository;

impl MasterCourseRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course: &NewMasterCourse,
    ) -> QueryResult<MasterCourse> {
        diesel::insert_into(master_course)
            .values(new_course)
            .returning(MasterCourse::as_returning())
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<MasterCourse>> {
        master_course.select(MasterCourse::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, course_id: i64) -> QueryResult<MasterCourse> {
        master_course
            .filter(id.eq(course_id))
            .select(MasterCourse::as_select())
            .first(conn)
    }

    pub fn find_by_course_code(
        conn: &mut PgConnection,
        target_course_code: &str,
    ) -> QueryResult<MasterCourse> {
        master_course
            .filter(course_code.eq(target_course_code))
            .select(MasterCourse::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        update_course: &UpdateMasterCourse,
    ) -> QueryResult<MasterCourse> {
        diesel::update(master_course.filter(id.eq(course_id)))
            .set(update_course)
            .returning(MasterCourse::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> QueryResult<usize> {
        diesel::delete(master_course.filter(id.eq(course_id))).execute(conn)
    }
}

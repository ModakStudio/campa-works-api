use diesel::prelude::*;

use crate::{
    models::{
        course_pool::{CoursePool, NewCoursePool},
        master_course::MasterCourse,
        professor::Professor,
        user::User,
    },
    schema::{course_pool, master_course, professor, users},
};

pub struct CoursePoolRepository;

impl CoursePoolRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course_pool: NewCoursePool,
    ) -> QueryResult<CoursePool> {
        diesel::insert_into(course_pool::table)
            .values(&new_course_pool)
            .returning(CoursePool::as_returning())
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<CoursePool>> {
        course_pool::table
            .inner_join(professor::table)
            .inner_join(users::table.on(users::id.eq(professor::user_id)))
            .inner_join(master_course::table)
            .select(CoursePool::as_select())
            .load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, course_pool_id: i64) -> QueryResult<CoursePool> {
        course_pool::table
            .inner_join(professor::table)
            .inner_join(users::table.on(users::id.eq(professor::user_id)))
            .inner_join(master_course::table)
            .filter(course_pool::id.eq(course_pool_id))
            .select(CoursePool::as_select())
            .first(conn)
    }
}

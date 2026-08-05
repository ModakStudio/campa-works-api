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
        new_course_pool: &NewCoursePool,
    ) -> QueryResult<CoursePool> {
        diesel::insert_into(course_pool::table)
            .values(new_course_pool)
            .returning(CoursePool::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(CoursePool, Professor, User, MasterCourse)>> {
        course_pool::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .select((
                CoursePool::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_pool_id: i64,
    ) -> QueryResult<(CoursePool, Professor, User, MasterCourse)> {
        course_pool::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_pool::id.eq(course_pool_id))
            .select((
                CoursePool::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_professor_id_and_master_course_id(
        conn: &mut PgConnection,
        professor_id: i64,
        master_course_id: i64,
    ) -> QueryResult<(CoursePool, Professor, User, MasterCourse)> {
        course_pool::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_pool::professor_id.eq(professor_id))
            .filter(course_pool::master_course_id.eq(master_course_id))
            .select((
                CoursePool::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_pool_id: i64) -> QueryResult<usize> {
        diesel::delete(course_pool::table.filter(course_pool::id.eq(course_pool_id))).execute(conn)
    }
}

use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course_pool::{CoursePool, NewCoursePool},
        enums::*,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{course_pool, master_course, professor, semester, users},
};

#[macro_export]
macro_rules! apply_course_pool_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_pool_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_pool::id.eq(course_pool_id));
        }

        query = crate::apply_professor_query_filters!(query, $params);

        query = crate::apply_master_course_query_filters!(query, $params);

        query
    }};
}

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
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CoursePool, Professor, User, Semester, MasterCourse)>> {
        let mut query = course_pool::table
            .inner_join(
                professor::table
                    .inner_join(users::table)
                    .inner_join(semester::table),
            )
            .inner_join(master_course::table)
            .select((
                CoursePool::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
                MasterCourse::as_select(),
            ))
            .into_boxed();

        query = apply_course_pool_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_pool_id: i64,
    ) -> QueryResult<(CoursePool, Professor, User, Semester, MasterCourse)> {
        course_pool::table
            .inner_join(
                professor::table
                    .inner_join(users::table)
                    .inner_join(semester::table),
            )
            .inner_join(master_course::table)
            .filter(course_pool::id.eq(course_pool_id))
            .select((
                CoursePool::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_pool_id: i64) -> QueryResult<usize> {
        diesel::delete(course_pool::table.filter(course_pool::id.eq(course_pool_id))).execute(conn)
    }
}

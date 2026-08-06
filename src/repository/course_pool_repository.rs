use diesel::prelude::*;
use std::collections::HashMap;

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
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CoursePool, Professor, User, MasterCourse)>> {
        let mut query = course_pool::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .into_boxed();

        if let Some(course_pool_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(course_pool::id.eq(course_pool_id));
        }

        if let Some(professor_id) = params
            .get("professor_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_pool::professor_id.eq(professor_id));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_pool::master_course_id.eq(master_course_id));
        }

        if let Some(professor_name) = params
            .get("professor_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::name.ilike(format!("%{}%", professor_name)));
        }

        if let Some(course_code) = params
            .get("course_code")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::course_code.ilike(format!("%{}%", course_code)));
        }

        if let Some(course_name) = params
            .get("course_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::name.ilike(format!("%{}%", course_name)));
        }

        query
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

    pub fn delete(conn: &mut PgConnection, course_pool_id: i64) -> QueryResult<usize> {
        diesel::delete(course_pool::table.filter(course_pool::id.eq(course_pool_id))).execute(conn)
    }
}

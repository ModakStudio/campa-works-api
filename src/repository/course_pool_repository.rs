use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course_pool::{CoursePool, NewCoursePool},
        enums::{CourseType, ProfessorPosition, ProfessorStatus},
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
        if let Some(professor_position) = params
            .get("professor_position")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(professor::position.eq(ProfessorPosition::from(professor_position)));
        }
        if let Some(professor_office) = params
            .get("professor_office")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::office.ilike(format!("%{}%", professor_office)));
        }
        if let Some(professor_tel) = params
            .get("professor_tel")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::tel.ilike(format!("%{}%", professor_tel)));
        }
        if let Some(professor_research_field) = params
            .get("professor_research_field")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(professor::research_field.ilike(format!("%{}%", professor_research_field)));
        }
        if let Some(professor_status) = params
            .get("professor_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::status.eq(ProfessorStatus::from(professor_status)));
        }

        if let Some(user_id) = params
            .get("user_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor::user_id.eq(user_id));
        }
        if let Some(user_email) = params
            .get("user_email")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::email.eq(user_email));
        }
        if let Some(user_name) = params
            .get("user_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::name.ilike(format!("%{}%", user_name)));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_pool::master_course_id.eq(master_course_id));
        }
        if let Some(master_course_course_code) = params.get("master_course_course_code") {
            query = query.filter(master_course::course_code.eq(master_course_course_code));
        }
        if let Some(master_course_name) = params
            .get("master_course_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::name.ilike(format!("%{}%", master_course_name)));
        }
        if let Some(master_course_credit) = params
            .get("master_course_credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::credit.eq(master_course_credit));
        }
        if let Some(master_course_lecture) = params
            .get("master_course_lecture")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::lecture.eq(master_course_lecture));
        }
        if let Some(master_course_practice) = params
            .get("master_course_practice")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::practice.eq(master_course_practice));
        }
        if let Some(master_course_course_type) = params
            .get("master_course_course_type")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(master_course::course_type.eq(CourseType::from(master_course_course_type)));
        }
        if let Some(master_course_is_core) = params
            .get("master_course_is_core")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(master_course::is_core.eq(master_course_is_core));
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

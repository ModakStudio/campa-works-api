use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course_preference::{CoursePreference, NewCoursePreference, UpdateCoursePreference},
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{course_preference, master_course, professor, semester, users},
};

pub struct CoursePreferenceRepository;

impl CoursePreferenceRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course_preference: &NewCoursePreference,
    ) -> QueryResult<CoursePreference> {
        diesel::insert_into(course_preference::table)
            .values(new_course_preference)
            .returning(CoursePreference::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CoursePreference, Semester, Professor, User, MasterCourse)>> {
        let mut query = course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .into_boxed();

        if let Some(course_preference_id) =
            params.get("id").and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::id.eq(course_preference_id));
        }

        if let Some(semester_id) = params
            .get("semester_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::semester_id.eq(semester_id));
        }

        if let Some(professor_id) = params
            .get("professor_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::professor_id.eq(professor_id));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::master_course_id.eq(master_course_id));
        }

        if let Some(priority) = params
            .get("priority")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course_preference::priority.eq(priority));
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
                CoursePreference::as_select(),
                Semester::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_preference_id: i64,
    ) -> QueryResult<(CoursePreference, Semester, Professor, User, MasterCourse)> {
        course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference::id.eq(course_preference_id))
            .select((
                CoursePreference::as_select(),
                Semester::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_preference_id: i64,
        update_course_preference: &UpdateCoursePreference,
    ) -> QueryResult<CoursePreference> {
        diesel::update(
            course_preference::table.filter(course_preference::id.eq(course_preference_id)),
        )
        .set(update_course_preference)
        .returning(CoursePreference::as_returning())
        .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_preference_id: i64) -> QueryResult<usize> {
        diesel::delete(
            course_preference::table.filter(course_preference::id.eq(course_preference_id)),
        )
        .execute(conn)
    }
}

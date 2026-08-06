use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course_preference_bookmark::{CoursePreferenceBookmark, NewCoursePreferenceBookmark},
        master_course::MasterCourse,
        professor::Professor,
        user::User,
    },
    schema::{course_preference_bookmark, master_course, professor, users},
};

pub struct CoursePreferenceBookmarkRepository;

impl CoursePreferenceBookmarkRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_bookmark: &NewCoursePreferenceBookmark,
    ) -> QueryResult<CoursePreferenceBookmark> {
        diesel::insert_into(course_preference_bookmark::table)
            .values(new_bookmark)
            .returning(CoursePreferenceBookmark::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CoursePreferenceBookmark, Professor, User, MasterCourse)>> {
        let mut query = course_preference_bookmark::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .into_boxed();

        if let Some(course_preference_bookmark_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(course_preference_bookmark::id.eq(course_preference_bookmark_id));
        }

        if let Some(professor_id) = params
            .get("professor_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference_bookmark::professor_id.eq(professor_id));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference_bookmark::master_course_id.eq(master_course_id));
        }

        query.select((
                CoursePreferenceBookmark::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_preference_bookmark_id: i64,
    ) -> QueryResult<(CoursePreferenceBookmark, Professor, User, MasterCourse)> {
        course_preference_bookmark::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference_bookmark::id.eq(course_preference_bookmark_id))
            .select((
                CoursePreferenceBookmark::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(
        conn: &mut PgConnection,
        course_preference_bookmark_id: i64,
    ) -> QueryResult<usize> {
        diesel::delete(
            course_preference_bookmark::table
                .filter(course_preference_bookmark::id.eq(course_preference_bookmark_id)),
        )
        .execute(conn)
    }
}

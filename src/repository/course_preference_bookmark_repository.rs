use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course_preference_bookmark::{CoursePreferenceBookmark, NewCoursePreferenceBookmark},
        enums::*,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{course_preference_bookmark, master_course, professor, semester, users},
};

#[macro_export]
macro_rules! apply_course_preference_bookmark_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(bookmark_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference_bookmark::id.eq(bookmark_id));
        }

        query = crate::apply_professor_query_filters!(query, $params);

        query = crate::apply_master_course_query_filters!(query, $params);

        query
    }};
}

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
    ) -> QueryResult<
        Vec<(
            CoursePreferenceBookmark,
            Professor,
            User,
            Semester,
            MasterCourse,
        )>,
    > {
        let mut query = course_preference_bookmark::table
            .inner_join(
                professor::table
                    .inner_join(users::table)
                    .inner_join(semester::table),
            )
            .inner_join(master_course::table)
            .select((
                CoursePreferenceBookmark::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
                MasterCourse::as_select(),
            ))
            .into_boxed();

        query = apply_course_preference_bookmark_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_preference_bookmark_id: i64,
    ) -> QueryResult<(
        CoursePreferenceBookmark,
        Professor,
        User,
        Semester,
        MasterCourse,
    )> {
        course_preference_bookmark::table
            .inner_join(
                professor::table
                    .inner_join(users::table)
                    .inner_join(semester::table),
            )
            .inner_join(master_course::table)
            .filter(course_preference_bookmark::id.eq(course_preference_bookmark_id))
            .select((
                CoursePreferenceBookmark::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
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

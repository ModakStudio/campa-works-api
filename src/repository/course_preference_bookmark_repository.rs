use diesel::prelude::*;

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
    ) -> QueryResult<Vec<(CoursePreferenceBookmark, Professor, User, MasterCourse)>> {
        course_preference_bookmark::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .select((
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

    pub fn find_by_professor_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> QueryResult<Vec<(CoursePreferenceBookmark, Professor, User, MasterCourse)>> {
        course_preference_bookmark::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference_bookmark::professor_id.eq(professor_id))
            .select((
                CoursePreferenceBookmark::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_professor_id_and_master_course_id(
        conn: &mut PgConnection,
        professor_id: i64,
        master_course_id: i64,
    ) -> QueryResult<(CoursePreferenceBookmark, Professor, User, MasterCourse)> {
        course_preference_bookmark::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference_bookmark::professor_id.eq(professor_id))
            .filter(course_preference_bookmark::master_course_id.eq(master_course_id))
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

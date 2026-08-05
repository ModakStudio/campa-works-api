use diesel::prelude::*;

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
    ) -> QueryResult<Vec<(CoursePreference, Semester, Professor, User, MasterCourse)>> {
        course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
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

    pub fn find_by_semester_id_and_professor_id_and_master_course_id(
        conn: &mut PgConnection,
        semester_id: i64,
        professor_id: i64,
        master_course_id: i64,
    ) -> QueryResult<(CoursePreference, Semester, Professor, User, MasterCourse)> {
        course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference::semester_id.eq(semester_id))
            .filter(course_preference::professor_id.eq(professor_id))
            .filter(course_preference::master_course_id.eq(master_course_id))
            .select((
                CoursePreference::as_select(),
                Semester::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_semester_id_and_professor_id_and_priority(
        conn: &mut PgConnection,
        semester_id: i64,
        professor_id: i64,
        priority: i32,
    ) -> QueryResult<(CoursePreference, Semester, Professor, User, MasterCourse)> {
        course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference::semester_id.eq(semester_id))
            .filter(course_preference::professor_id.eq(professor_id))
            .filter(course_preference::priority.eq(priority))
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

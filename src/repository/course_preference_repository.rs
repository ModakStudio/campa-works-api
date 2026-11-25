use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course::Course,
        course_curriculum::CourseCurriculum,
        course_preference::{CoursePreference, NewCoursePreference, UpdateCoursePreference},
        curriculum::Curriculum,
        enums::*,
        major::Major,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{
        course, course_curriculum, course_preference, curriculum, major, master_course, professor,
        semester, users,
    },
};

#[macro_export]
macro_rules! apply_course_preference_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_preference_id) = $params
            .get("course_preference_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::id.eq(course_preference_id));
        }

        query = crate::apply_professor_query_filters!(query, $params);

        query = crate::apply_course_query_filters!(query, $params);

        if let Some(priority) = $params
            .get("priority")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course_preference::priority.eq(priority));
        }

        query
    }};
}

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
    ) -> QueryResult<
        Vec<(
            CoursePreference,
            Professor,
            User,
            Course,
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
        )>,
    > {
        let mut query = course_preference::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(
                course::table.inner_join(
                    course_curriculum::table
                        .inner_join(master_course::table)
                        .inner_join(
                            curriculum::table
                                .inner_join(semester::table)
                                .inner_join(major::table),
                        ),
                ),
            )
            .select((
                CoursePreference::as_select(),
                Professor::as_select(),
                User::as_select(),
                Course::as_select(),
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .into_boxed();

        query = apply_course_preference_query_filters!(query, params);

        query.load(conn)
    }

    // ToDo: Delete this function after the course_preference table is updated to remove semester_id and master_course_id columns
    pub fn find_by_professor_id_and_semester_id_and_master_course_id(
        conn: &mut PgConnection,
        professor_id: i64,
        semester_id: i64,
        master_course_id: i64,
    ) -> QueryResult<
        Vec<(
            CoursePreference,
            Professor,
            User,
            Course,
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
        )>,
    > {
        course_preference::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(
                course::table.inner_join(
                    course_curriculum::table
                        .inner_join(master_course::table)
                        .inner_join(
                            curriculum::table
                                .inner_join(semester::table)
                                .inner_join(major::table),
                        ),
                ),
            )
            .select((
                CoursePreference::as_select(),
                Professor::as_select(),
                User::as_select(),
                Course::as_select(),
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_preference_id: i64,
    ) -> QueryResult<(
        CoursePreference,
        Professor,
        User,
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )> {
        course_preference::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(
                course::table.inner_join(
                    course_curriculum::table
                        .inner_join(master_course::table)
                        .inner_join(
                            curriculum::table
                                .inner_join(semester::table)
                                .inner_join(major::table),
                        ),
                ),
            )
            .filter(course_preference::id.eq(course_preference_id))
            .select((
                CoursePreference::as_select(),
                Professor::as_select(),
                User::as_select(),
                Course::as_select(),
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
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

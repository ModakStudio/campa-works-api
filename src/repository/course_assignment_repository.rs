use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course::Course,
        course_assignment::{CourseAssignment, NewCourseAssignment},
        enums::*,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{course, course_assignment, master_course, professor, semester, users},
};

#[macro_export]
macro_rules! apply_course_assignment_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_assignment_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_assignment::id.eq(course_assignment_id));
        }

        query = crate::apply_course_query_filters!(query, $params);

        query = crate::apply_professor_query_filters!(query, $params);

        query
    }};
}

pub struct CourseAssignmentRepository;

impl CourseAssignmentRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_assignment: &NewCourseAssignment,
    ) -> QueryResult<CourseAssignment> {
        diesel::insert_into(course_assignment::table)
            .values(new_assignment)
            .returning(CourseAssignment::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<
        Vec<(
            CourseAssignment,
            Course,
            MasterCourse,
            Professor,
            User,
            Semester,
        )>,
    > {
        let mut query = course_assignment::table
            .inner_join(course::table.inner_join(master_course::table))
            .inner_join(
                professor::table
                    .inner_join(users::table)
                    .inner_join(semester::table),
            )
            .select((
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .into_boxed();

        query = apply_course_assignment_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_assignment_id: i64,
    ) -> QueryResult<(
        CourseAssignment,
        Course,
        MasterCourse,
        Professor,
        User,
        Semester,
    )> {
        course_assignment::table
            .inner_join(course::table.inner_join(master_course::table))
            .inner_join(
                professor::table
                    .inner_join(users::table)
                    .inner_join(semester::table),
            )
            .filter(course_assignment::id.eq(course_assignment_id))
            .select((
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_assignment_id: i64) -> QueryResult<usize> {
        diesel::delete(
            course_assignment::table.filter(course_assignment::id.eq(course_assignment_id)),
        )
        .execute(conn)
    }
}

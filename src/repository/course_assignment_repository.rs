use diesel::prelude::*;

use crate::{
    models::{
        course::Course,
        course_assignment::{CourseAssignment, NewCourseAssignment},
        major::Major,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{course, course_assignment, major, master_course, professor, semester, users},
};

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
    ) -> QueryResult<
        Vec<(
            CourseAssignment,
            Course,
            MasterCourse,
            Semester,
            Major,
            Professor,
            User,
        )>,
    > {
        course_assignment::table
            .inner_join(
                course::table
                    .inner_join(master_course::table)
                    .inner_join(semester::table)
                    .inner_join(major::table),
            )
            .inner_join(professor::table.inner_join(users::table))
            .select((
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_assignment_id: i64,
    ) -> QueryResult<(
        CourseAssignment,
        Course,
        MasterCourse,
        Semester,
        Major,
        Professor,
        User,
    )> {
        course_assignment::table
            .inner_join(
                course::table
                    .inner_join(master_course::table)
                    .inner_join(semester::table)
                    .inner_join(major::table),
            )
            .inner_join(professor::table.inner_join(users::table))
            .filter(course_assignment::id.eq(course_assignment_id))
            .select((
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_course_id(
        conn: &mut PgConnection,
        course_id: i64,
    ) -> QueryResult<(
        CourseAssignment,
        Course,
        MasterCourse,
        Semester,
        Major,
        Professor,
        User,
    )> {
        course_assignment::table
            .inner_join(
                course::table
                    .inner_join(master_course::table)
                    .inner_join(semester::table)
                    .inner_join(major::table),
            )
            .inner_join(professor::table.inner_join(users::table))
            .filter(course_assignment::course_id.eq(course_id))
            .select((
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_professor_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> QueryResult<
        Vec<(
            CourseAssignment,
            Course,
            MasterCourse,
            Semester,
            Major,
            Professor,
            User,
        )>,
    > {
        course_assignment::table
            .inner_join(
                course::table
                    .inner_join(master_course::table)
                    .inner_join(semester::table)
                    .inner_join(major::table),
            )
            .inner_join(professor::table.inner_join(users::table))
            .filter(course_assignment::professor_id.eq(professor_id))
            .select((
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
            ))
            .load(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_assignment_id: i64) -> QueryResult<usize> {
        diesel::delete(
            course_assignment::table.filter(course_assignment::id.eq(course_assignment_id)),
        )
        .execute(conn)
    }
}

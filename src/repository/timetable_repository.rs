use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        classroom::Classroom,
        course::Course,
        course_assignment::CourseAssignment,
        major::Major,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        timetable::{NewTimetable, Timetable, UpdateTimetable},
        user::User,
    },
    schema::{
        classroom, course, course_assignment, major, master_course, professor, semester, timetable,
        users,
    },
};

pub struct TimetableRepository;

impl TimetableRepository {
    pub fn create(conn: &mut PgConnection, new_timetable: &NewTimetable) -> QueryResult<Timetable> {
        diesel::insert_into(timetable::table)
            .values(new_timetable)
            .returning(Timetable::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<
        Vec<(
            Timetable,
            CourseAssignment,
            Course,
            MasterCourse,
            Semester,
            Major,
            Professor,
            User,
            Classroom,
        )>,
    > {
        let mut query = timetable::table
            .inner_join(
                course_assignment::table
                    .inner_join(
                        course::table
                            .inner_join(master_course::table)
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    )
                    .inner_join(professor::table.inner_join(users::table)),
            )
            .inner_join(classroom::table)
            .into_boxed();

        if let Some(timetable_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(timetable::id.eq(timetable_id));
        }

        if let Some(assignment_id) = params
            .get("assignment_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(timetable::assignment_id.eq(assignment_id));
        }

        if let Some(classroom_id) = params
            .get("classroom_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(timetable::classroom_id.eq(classroom_id));
        }

        if let Some(day_of_week) = params
            .get("day_of_week")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(timetable::day_of_week.eq(day_of_week));
        }

        if let Some(professor_name) = params
            .get("professor_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::name.ilike(format!("%{}%", professor_name)));
        }

        query
            .select((
                Timetable::as_select(),
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
                Classroom::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        timetable_id: i64,
    ) -> QueryResult<(
        Timetable,
        CourseAssignment,
        Course,
        MasterCourse,
        Semester,
        Major,
        Professor,
        User,
        Classroom,
    )> {
        timetable::table
            .inner_join(
                course_assignment::table
                    .inner_join(
                        course::table
                            .inner_join(master_course::table)
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    )
                    .inner_join(professor::table.inner_join(users::table)),
            )
            .inner_join(classroom::table)
            .filter(timetable::id.eq(timetable_id))
            .select((
                Timetable::as_select(),
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
                Classroom::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        timetable_id: i64,
        update_timetable: &UpdateTimetable,
    ) -> QueryResult<Timetable> {
        diesel::update(timetable::table.filter(timetable::id.eq(timetable_id)))
            .set(update_timetable)
            .returning(Timetable::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, timetable_id: i64) -> QueryResult<usize> {
        diesel::delete(timetable::table.filter(timetable::id.eq(timetable_id))).execute(conn)
    }
}

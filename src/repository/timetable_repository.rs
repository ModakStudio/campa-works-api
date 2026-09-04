use std::collections::HashMap;

use chrono::NaiveTime;
use diesel::prelude::*;

use crate::{
    models::{
        classroom::Classroom,
        course::Course,
        course_assignment::CourseAssignment,
        enums::*,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        timetable::{NewTimetable, Timetable, UpdateTimetable},
        user::User,
    },
    schema::{
        classroom, course, course_assignment, master_course, professor, semester, timetable, users,
    },
};

#[macro_export]
macro_rules! apply_timetable_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(timetable_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(timetable::id.eq(timetable_id));
        }

        query = crate::apply_course_assignment_query_filters!(query, $params);

        query = crate::apply_classroom_query_filters!(query, $params);

        if let Some(day_of_week) = $params
            .get("day_of_week")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(timetable::day_of_week.eq(DayOfWeek::from(day_of_week)));
        }
        if let Some(start_time) = $params
            .get("start_time")
            .and_then(|value| value.parse::<NaiveTime>().ok())
        {
            query = query.filter(timetable::start_time.eq(start_time));
        }
        if let Some(end_time) = $params
            .get("end_time")
            .and_then(|value| value.parse::<NaiveTime>().ok())
        {
            query = query.filter(timetable::end_time.eq(end_time));
        }

        query
    }};
}

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
            Professor,
            User,
            Semester,
            Classroom,
        )>,
    > {
        let mut query = timetable::table
            .inner_join(
                course_assignment::table
                    .inner_join(course::table.inner_join(master_course::table))
                    .inner_join(
                        professor::table
                            .inner_join(users::table)
                            .inner_join(semester::table),
                    ),
            )
            .inner_join(classroom::table)
            .select((
                Timetable::as_select(),
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
                Classroom::as_select(),
            ))
            .into_boxed();

        query = apply_timetable_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        timetable_id: i64,
    ) -> QueryResult<(
        Timetable,
        CourseAssignment,
        Course,
        MasterCourse,
        Professor,
        User,
        Semester,
        Classroom,
    )> {
        timetable::table
            .inner_join(
                course_assignment::table
                    .inner_join(course::table.inner_join(master_course::table))
                    .inner_join(
                        professor::table
                            .inner_join(users::table)
                            .inner_join(semester::table),
                    ),
            )
            .inner_join(classroom::table)
            .filter(timetable::id.eq(timetable_id))
            .select((
                Timetable::as_select(),
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
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

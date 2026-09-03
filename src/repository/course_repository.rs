use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course::{Course, NewCourse, UpdateCourse},
        enums::*,
        master_course::MasterCourse,
    },
    schema::{course, master_course},
};

#[macro_export]
macro_rules! apply_course_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::id.eq(course_id));
        }

        query = crate::apply_master_course_query_filters!(query, $params);

        if let Some(course_description) = $params
            .get("course_description")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(course::course_description.ilike(format!("%{}%", course_description)));
        }
        if let Some(grade) = $params
            .get("grade")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::grade.eq(grade));
        }
        if let Some(credit) = $params
            .get("credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::credit.eq(credit));
        }
        if let Some(lecture) = $params
            .get("lecture")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::lecture.eq(lecture));
        }
        if let Some(practice) = $params
            .get("practice")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::practice.eq(practice));
        }
        if let Some(course_category) = $params
            .get("course_category")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::course_category.eq(CourseCategory::from(course_category)));
        }
        if let Some(language) = $params
            .get("language")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::language.eq(Language::from(language)));
        }
        if let Some(section_number) = $params
            .get("section_number")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::section_number.eq(section_number));
        }
        if let Some(capacity) = $params
            .get("capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::capacity.eq(capacity));
        }
        if let Some(participant) = $params
            .get("participant")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::participant.eq(participant));
        }

        query
    }};
}

pub struct CourseRepository;

impl CourseRepository {
    pub fn create(conn: &mut PgConnection, new_course: &NewCourse) -> QueryResult<Course> {
        diesel::insert_into(course::table)
            .values(new_course)
            .returning(Course::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(Course, MasterCourse)>> {
        let mut query = course::table
            .inner_join(master_course::table)
            .select((Course::as_select(), MasterCourse::as_select()))
            .into_boxed();

        query = apply_course_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_id: i64,
    ) -> QueryResult<(Course, MasterCourse)> {
        course::table
            .inner_join(master_course::table)
            .filter(course::id.eq(course_id))
            .select((Course::as_select(), MasterCourse::as_select()))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        update_course: &UpdateCourse,
    ) -> QueryResult<Course> {
        diesel::update(course::table.filter(course::id.eq(course_id)))
            .set(update_course)
            .returning(Course::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> QueryResult<usize> {
        diesel::delete(course::table.filter(course::id.eq(course_id))).execute(conn)
    }
}

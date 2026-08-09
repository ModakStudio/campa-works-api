use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course::{Course, NewCourse, UpdateCourse},
        enums::{CourseCategory, CourseType, Language, SemesterStatus, SemesterType},
        major::Major,
        master_course::MasterCourse,
        semester::Semester,
    },
    schema::{course, major, master_course, semester},
};

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
    ) -> QueryResult<Vec<(Course, MasterCourse, Semester, Major)>> {
        let mut query = course::table
            .inner_join(master_course::table)
            .inner_join(semester::table)
            .inner_join(major::table)
            .into_boxed();

        if let Some(course_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(course::id.eq(course_id));
        }
        if let Some(description) = params
            .get("description")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::description.ilike(format!("%{}%", description)));
        }
        if let Some(course_category) = params
            .get("course_category")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::course_category.eq(CourseCategory::from(course_category)));
        }
        if let Some(language) = params
            .get("language")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::language.eq(Language::from(language)));
        }
        if let Some(section_number) = params
            .get("section_number")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::section_number.eq(section_number));
        }
        if let Some(grade) = params
            .get("grade")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::grade.eq(grade));
        }
        if let Some(capacity) = params
            .get("capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::capacity.eq(capacity));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::master_course_id.eq(master_course_id));
        }
        if let Some(master_course_course_code) = params
            .get("master_course_course_code")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(
                master_course::course_code.ilike(format!("%{}%", master_course_course_code)),
            );
        }
        if let Some(master_course_name) = params
            .get("master_course_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::name.ilike(format!("%{}%", master_course_name)));
        }
        if let Some(master_course_credit) = params
            .get("master_course_credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::credit.eq(master_course_credit));
        }
        if let Some(master_course_lecture) = params
            .get("master_course_lecture")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::lecture.eq(master_course_lecture));
        }
        if let Some(master_course_practice) = params
            .get("master_course_practice")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::practice.eq(master_course_practice));
        }
        if let Some(master_course_course_type) = params
            .get("master_course_course_type")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(master_course::course_type.eq(CourseType::from(master_course_course_type)));
        }
        if let Some(master_course_is_core) = params
            .get("master_course_is_core")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(master_course::is_core.eq(master_course_is_core));
        }

        if let Some(semester_id) = params
            .get("semester_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::semester_id.eq(semester_id));
        }
        if let Some(semester_year) = params
            .get("semester_year")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(semester::year.eq(semester_year));
        }
        if let Some(semester_semester_) = params
            .get("semester_semester_")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::semester_.eq(SemesterType::from(semester_semester_)));
        }
        if let Some(semester_status) = params
            .get("semester_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::status.eq(SemesterStatus::from(semester_status)));
        }

        if let Some(major_id) = params
            .get("major_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::major_id.eq(major_id));
        }
        if let Some(major_name) = params
            .get("major_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(major::name.ilike(format!("%{}%", major_name)));
        }

        query
            .select((
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_id: i64,
    ) -> QueryResult<(Course, MasterCourse, Semester, Major)> {
        course::table
            .inner_join(master_course::table)
            .inner_join(semester::table)
            .inner_join(major::table)
            .filter(course::id.eq(course_id))
            .select((
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
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

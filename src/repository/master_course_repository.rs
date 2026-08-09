use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        enums::CourseType,
        master_course::{MasterCourse, NewMasterCourse, UpdateMasterCourse},
    },
    schema::master_course,
};

pub struct MasterCourseRepository;

impl MasterCourseRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course: &NewMasterCourse,
    ) -> QueryResult<MasterCourse> {
        diesel::insert_into(master_course::table)
            .values(new_course)
            .returning(MasterCourse::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<MasterCourse>> {
        let mut query = master_course::table
            .select(MasterCourse::as_select())
            .into_boxed();

        if let Some(id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(master_course::id.eq(id));
        }
        if let Some(course_code) = params.get("course_code") {
            query = query.filter(master_course::course_code.eq(course_code));
        }
        if let Some(name) = params
            .get("name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::name.ilike(format!("%{}%", name)));
        }
        if let Some(credit) = params
            .get("credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::credit.eq(credit));
        }
        if let Some(lecture) = params
            .get("lecture")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::lecture.eq(lecture));
        }
        if let Some(practice) = params
            .get("practice")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::practice.eq(practice));
        }
        if let Some(course_type) = params
            .get("course_type")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::course_type.eq(CourseType::from(course_type)));
        }
        if let Some(is_core) = params
            .get("is_core")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(master_course::is_core.eq(is_core));
        }

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, course_id: i64) -> QueryResult<MasterCourse> {
        master_course::table
            .filter(master_course::id.eq(course_id))
            .select(MasterCourse::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        update_course: &UpdateMasterCourse,
    ) -> QueryResult<MasterCourse> {
        diesel::update(master_course::table.filter(master_course::id.eq(course_id)))
            .set(update_course)
            .returning(MasterCourse::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> QueryResult<usize> {
        diesel::delete(master_course::table.filter(master_course::id.eq(course_id))).execute(conn)
    }
}

use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        enums::*,
        master_course::{MasterCourse, NewMasterCourse, UpdateMasterCourse},
    },
    schema::master_course,
};

#[macro_export]
macro_rules! apply_master_course_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(master_course_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(master_course::id.eq(master_course_id));
        }
        if let Some(course_code) = $params
            .get("course_code")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::course_code.eq(course_code));
        }
        if let Some(course_name) = $params
            .get("course_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::course_name.ilike(format!("%{}%", course_name)));
        }
        if let Some(course_en_name) = $params
            .get("course_en_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(master_course::course_en_name.ilike(format!("%{}%", course_en_name)));
        }
        if let Some(course_type) = $params
            .get("course_type")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::course_type.eq(CourseType::from(course_type)));
        }
        if let Some(is_core) = $params
            .get("is_core")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(master_course::is_core.eq(is_core));
        }
        if let Some(course_status) = $params
            .get("course_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(master_course::course_status.eq(CourseStatus::from(course_status)));
        }

        query
    }};
}

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

        query = apply_master_course_query_filters!(query, params);

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

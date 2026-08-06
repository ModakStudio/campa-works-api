use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course_facility::{CourseFacility, NewCourseFacility},
        facility::Facility,
        master_course::MasterCourse,
    },
    schema::{course_facility, facility, master_course},
};

pub struct CourseFacilityRepository;

impl CourseFacilityRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course_facility: &NewCourseFacility,
    ) -> QueryResult<CourseFacility> {
        diesel::insert_into(course_facility::table)
            .values(new_course_facility)
            .returning(CourseFacility::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CourseFacility, MasterCourse, Facility)>> {
        let mut query = course_facility::table
            .inner_join(master_course::table)
            .inner_join(facility::table)
            .into_boxed();

        if let Some(course_facility_id) =
            params.get("id").and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_facility::id.eq(course_facility_id));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_facility::master_course_id.eq(master_course_id));
        }

        if let Some(facility_id) = params
            .get("facility_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_facility::facility_id.eq(facility_id));
        }

        if let Some(facility_name) = params
            .get("facility_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(facility::name.ilike(format!("%{}%", facility_name)));
        }

        query
            .select((
                CourseFacility::as_select(),
                MasterCourse::as_select(),
                Facility::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_facility_id: i64,
    ) -> QueryResult<(CourseFacility, MasterCourse, Facility)> {
        course_facility::table
            .inner_join(master_course::table)
            .inner_join(facility::table)
            .filter(course_facility::id.eq(course_facility_id))
            .select((
                CourseFacility::as_select(),
                MasterCourse::as_select(),
                Facility::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_facility_id: i64) -> QueryResult<usize> {
        diesel::delete(course_facility::table.filter(course_facility::id.eq(course_facility_id)))
            .execute(conn)
    }
}

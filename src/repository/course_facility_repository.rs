use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course_facility::{CourseFacility, NewCourseFacility},
        enums::*,
        facility::Facility,
        master_course::MasterCourse,
    },
    schema::{course_facility, facility, master_course},
};

#[macro_export]
macro_rules! apply_course_facility_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_facility_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_facility::id.eq(course_facility_id));
        }

        query = crate::apply_master_course_query_filters!(query, $params);

        query = crate::apply_facility_query_filters!(query, $params);

        query
    }};
}

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
            .select((
                CourseFacility::as_select(),
                MasterCourse::as_select(),
                Facility::as_select(),
            ))
            .into_boxed();

        query = apply_course_facility_query_filters!(query, params);

        query.load(conn)
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

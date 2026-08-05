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
    ) -> QueryResult<Vec<(CourseFacility, MasterCourse, Facility)>> {
        course_facility::table
            .inner_join(master_course::table)
            .inner_join(facility::table)
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

    pub fn find_by_master_course_id_and_facility_id(
        conn: &mut PgConnection,
        target_master_course_id: i64,
        target_facility_id: i64,
    ) -> QueryResult<(CourseFacility, MasterCourse, Facility)> {
        course_facility::table
            .inner_join(master_course::table)
            .inner_join(facility::table)
            .filter(course_facility::master_course_id.eq(target_master_course_id))
            .filter(course_facility::facility_id.eq(target_facility_id))
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

use diesel::prelude::*;

use crate::{
    models::{
        classroom::Classroom,
        classroom_facility::{ClassroomFacility, NewClassroomFacility},
        facility::Facility,
    },
    schema::{classroom, classroom_facility, facility},
};

pub struct ClassroomFacilityRepository;

impl ClassroomFacilityRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_classroom_facility: &NewClassroomFacility,
    ) -> QueryResult<ClassroomFacility> {
        diesel::insert_into(classroom_facility::table)
            .values(new_classroom_facility)
            .returning(ClassroomFacility::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(ClassroomFacility, Classroom, Facility)>> {
        classroom_facility::table
            .inner_join(classroom::table)
            .inner_join(facility::table)
            .select((
                ClassroomFacility::as_select(),
                Classroom::as_select(),
                Facility::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        classroom_facility_id: i64,
    ) -> QueryResult<(ClassroomFacility, Classroom, Facility)> {
        classroom_facility::table
            .inner_join(classroom::table)
            .inner_join(facility::table)
            .filter(classroom_facility::id.eq(classroom_facility_id))
            .select((
                ClassroomFacility::as_select(),
                Classroom::as_select(),
                Facility::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_classroom_id_and_facility_id(
        conn: &mut PgConnection,
        target_classroom_id: i64,
        target_facility_id: i64,
    ) -> QueryResult<(ClassroomFacility, Classroom, Facility)> {
        classroom_facility::table
            .inner_join(classroom::table)
            .inner_join(facility::table)
            .filter(classroom_facility::classroom_id.eq(target_classroom_id))
            .filter(classroom_facility::facility_id.eq(target_facility_id))
            .select((
                ClassroomFacility::as_select(),
                Classroom::as_select(),
                Facility::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, classroom_facility_id: i64) -> QueryResult<usize> {
        diesel::delete(
            classroom_facility::table.filter(classroom_facility::id.eq(classroom_facility_id)),
        )
        .execute(conn)
    }
}
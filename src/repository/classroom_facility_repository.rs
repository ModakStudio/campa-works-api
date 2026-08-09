use std::collections::HashMap;

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
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(ClassroomFacility, Classroom, Facility)>> {
        let mut query = classroom_facility::table
            .inner_join(classroom::table)
            .inner_join(facility::table)
            .into_boxed();

        if let Some(classroom_facility_id) =
            params.get("id").and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(classroom_facility::id.eq(classroom_facility_id));
        }

        if let Some(classroom_id) = params
            .get("classroom_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(classroom_facility::classroom_id.eq(classroom_id));
        }
        if let Some(classroom_building) = params
            .get("classroom_building")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(classroom::building.ilike(format!("%{}%", classroom_building)));
        }
        if let Some(classroom_room) = params
            .get("classroom_room")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(classroom::room.ilike(format!("%{}%", classroom_room)));
        }
        if let Some(classroom_capacity) = params
            .get("classroom_capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(classroom::capacity.eq(classroom_capacity));
        }
        if let Some(classroom_is_available) = params
            .get("classroom_is_available")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(classroom::is_available.eq(classroom_is_available));
        }

        if let Some(facility_id) = params
            .get("facility_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(classroom_facility::facility_id.eq(facility_id));
        }
        if let Some(facility_name) = params
            .get("facility_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(facility::name.ilike(format!("%{}%", facility_name)));
        }
        if let Some(facility_description) = params
            .get("facility_description")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(facility::description.ilike(format!("%{}%", facility_description)));
        }

        query
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

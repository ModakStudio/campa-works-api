use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::facility::{Facility, NewFacility, UpdateFacility},
    schema::facility,
};

pub struct FacilityRepository;

impl FacilityRepository {
    pub fn create(conn: &mut PgConnection, new_facility: &NewFacility) -> QueryResult<Facility> {
        diesel::insert_into(facility::table)
            .values(new_facility)
            .returning(Facility::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<Facility>> {
        let mut query = facility::table.select(Facility::as_select()).into_boxed();

        if let Some(facility_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(facility::id.eq(facility_id));
        }

        if let Some(name) = params
            .get("name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(facility::name.ilike(format!("%{}%", name)));
        }

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, facility_id: i64) -> QueryResult<Facility> {
        facility::table
            .filter(facility::id.eq(facility_id))
            .select(Facility::as_select())
            .first(conn)
    }

    pub fn find_by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Facility> {
        facility::table
            .filter(facility::name.eq(name))
            .select(Facility::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        facility_id: i64,
        update_facility: &UpdateFacility,
    ) -> QueryResult<Facility> {
        diesel::update(facility::table.filter(facility::id.eq(facility_id)))
            .set(update_facility)
            .returning(Facility::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, facility_id: i64) -> QueryResult<usize> {
        diesel::delete(facility::table.filter(facility::id.eq(facility_id))).execute(conn)
    }
}

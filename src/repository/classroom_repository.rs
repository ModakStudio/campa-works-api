use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::classroom::{Classroom, NewClassroom, UpdateClassroom},
    schema::classroom,
};

pub struct ClassroomRepository;

impl ClassroomRepository {
    pub fn create(conn: &mut PgConnection, new_classroom: &NewClassroom) -> QueryResult<Classroom> {
        diesel::insert_into(classroom::table)
            .values(new_classroom)
            .returning(Classroom::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<Classroom>> {
        let mut query = classroom::table.select(Classroom::as_select()).into_boxed();

        if let Some(classroom_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(classroom::id.eq(classroom_id));
        }

        if let Some(building) = params
            .get("building")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(classroom::building.ilike(format!("%{}%", building)));
        }

        if let Some(room) = params
            .get("room")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(classroom::room.ilike(format!("%{}%", room)));
        }

        if let Some(capacity) = params
            .get("capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(classroom::capacity.eq(capacity));
        }

        if let Some(is_available) = params
            .get("is_available")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(classroom::is_available.eq(is_available));
        }

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, classroom_id: i64) -> QueryResult<Classroom> {
        classroom::table
            .filter(classroom::id.eq(classroom_id))
            .select(Classroom::as_select())
            .first(conn)
    }

    pub fn find_by_building_and_room(
        conn: &mut PgConnection,
        building: &str,
        room: &str,
    ) -> QueryResult<Classroom> {
        classroom::table
            .filter(classroom::building.eq(building))
            .filter(classroom::room.eq(room))
            .select(Classroom::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        classroom_id: i64,
        update_classroom: &UpdateClassroom,
    ) -> QueryResult<Classroom> {
        diesel::update(classroom::table.filter(classroom::id.eq(classroom_id)))
            .set(update_classroom)
            .returning(Classroom::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, classroom_id: i64) -> QueryResult<usize> {
        diesel::delete(classroom::table.filter(classroom::id.eq(classroom_id))).execute(conn)
    }
}

use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        enums::*,
        major::{Major, NewMajor, UpdateMajor},
    },
    schema::major,
};

#[macro_export]
macro_rules! apply_major_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(major_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(major::id.eq(major_id));
        }
        if let Some(major_name) = $params
            .get("major_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(major::major_name.ilike(format!("%{}%", major_name)));
        }
        if let Some(major_code) = $params
            .get("major_code")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(major::major_code.ilike(format!("%{}%", major_code)));
        }
        if let Some(major_status) = $params
            .get("major_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(major::major_status.eq(MajorStatus::from(major_status)));
        }

        query
    }};
}

pub struct MajorRepository;

impl MajorRepository {
    pub fn create(conn: &mut PgConnection, new_major: &NewMajor) -> QueryResult<Major> {
        diesel::insert_into(major::table)
            .values(new_major)
            .returning(Major::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<Major>> {
        let mut query = major::table.select(Major::as_select()).into_boxed();

        query = apply_major_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, major_id: i64) -> QueryResult<Major> {
        major::table
            .filter(major::id.eq(major_id))
            .select(Major::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        major_id: i64,
        update_major: &UpdateMajor,
    ) -> QueryResult<Major> {
        diesel::update(major::table.filter(major::id.eq(major_id)))
            .set(update_major)
            .returning(Major::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, major_id: i64) -> QueryResult<usize> {
        diesel::delete(major::table.filter(major::id.eq(major_id))).execute(conn)
    }
}

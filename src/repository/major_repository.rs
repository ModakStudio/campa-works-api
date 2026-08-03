use diesel::prelude::*;

use crate::{
    models::major::{Major, NewMajor, UpdateMajor},
    schema::major::dsl::*,
};

pub struct MajorRepository;

impl MajorRepository {
    pub fn create(conn: &mut PgConnection, new_major: &NewMajor) -> QueryResult<Major> {
        diesel::insert_into(major)
            .values(new_major)
            .returning(Major::as_returning())
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Major>> {
        major.select(Major::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, major_id: i64) -> QueryResult<Major> {
        major
            .filter(id.eq(major_id))
            .select(Major::as_select())
            .first(conn)
    }

    pub fn find_by_name(conn: &mut PgConnection, major_name: &str) -> QueryResult<Major> {
        major
            .filter(name.eq(major_name))
            .select(Major::as_select())
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        major_id: i64,
        update_major: &UpdateMajor,
    ) -> QueryResult<Major> {
        diesel::update(major.filter(id.eq(major_id)))
            .set(update_major)
            .returning(Major::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, major_id: i64) -> QueryResult<usize> {
        diesel::delete(major.filter(id.eq(major_id))).execute(conn)
    }
}

use diesel::prelude::*;

use crate::{
    models::{
        professor::{NewProfessor, Professor, UpdateProfessor},
        user::User,
    },
    schema::{professor, users},
};

pub struct ProfessorRepository;

impl ProfessorRepository {
    pub fn create(conn: &mut PgConnection, new_professor: &NewProfessor) -> QueryResult<Professor> {
        diesel::insert_into(professor::table)
            .values(new_professor)
            .returning(Professor::as_returning())
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<(Professor, User)>> {
        professor::table
            .inner_join(users::table)
            .select((Professor::as_select(), User::as_select()))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> QueryResult<(Professor, User)> {
        professor::table
            .inner_join(users::table)
            .filter(professor::id.eq(professor_id))
            .select((Professor::as_select(), User::as_select()))
            .first(conn)
    }

    pub fn find_by_user_id(
        conn: &mut PgConnection,
        target_user_id: i64,
    ) -> QueryResult<(Professor, User)> {
        professor::table
            .inner_join(users::table)
            .filter(professor::user_id.eq(target_user_id))
            .select((Professor::as_select(), User::as_select()))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_id: i64,
        update_professor: &UpdateProfessor,
    ) -> QueryResult<Professor> {
        diesel::update(professor::table.filter(professor::id.eq(professor_id)))
            .set(update_professor)
            .returning(Professor::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, professor_id: i64) -> QueryResult<usize> {
        diesel::delete(professor::table.filter(professor::id.eq(professor_id))).execute(conn)
    }
}

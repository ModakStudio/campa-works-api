use diesel::prelude::*;

use crate::{
    models::professor::{NewProfessor, Professor, UpdateProfessor},
    schema::professor::dsl::*,
};

pub struct ProfessorRepository;

impl ProfessorRepository {
    pub fn create(conn: &mut PgConnection, new_professor: &NewProfessor) -> QueryResult<Professor> {
        diesel::insert_into(professor)
            .values(new_professor)
            .returning(Professor::as_returning())
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Professor>> {
        professor.load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, professor_id: i64) -> QueryResult<Professor> {
        professor.filter(id.eq(professor_id)).first(conn)
    }

    pub fn find_by_user_id(conn: &mut PgConnection, target_user_id: i64) -> QueryResult<Professor> {
        professor.filter(user_id.eq(target_user_id)).first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_id: i64,
        update_professor: &UpdateProfessor,
    ) -> QueryResult<Professor> {
        diesel::update(professor.filter(id.eq(professor_id)))
            .set(update_professor)
            .returning(Professor::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, professor_id: i64) -> QueryResult<usize> {
        diesel::delete(professor.filter(id.eq(professor_id))).execute(conn)
    }
}

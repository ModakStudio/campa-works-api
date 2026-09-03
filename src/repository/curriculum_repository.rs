use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        curriculum::{Curriculum, NewCurriculum},
        enums::*,
        major::Major,
        semester::Semester,
    },
    schema::{curriculum, major, semester},
};

#[macro_export]
macro_rules! apply_curriculum_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(curriculum_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(curriculum::id.eq(curriculum_id));
        }

        query = crate::apply_semester_query_filters!(query, $params);

        query = crate::apply_major_query_filters!(query, $params);

        query
    }};
}

pub struct CurriculumRepository;

impl CurriculumRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_curriculum: &NewCurriculum,
    ) -> QueryResult<Curriculum> {
        diesel::insert_into(curriculum::table)
            .values(new_curriculum)
            .returning(Curriculum::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(Curriculum, Semester, Major)>> {
        let mut query = curriculum::table
            .inner_join(semester::table)
            .inner_join(major::table)
            .select((
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .into_boxed();

        query = apply_curriculum_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        curriculum_id: i64,
    ) -> QueryResult<(Curriculum, Semester, Major)> {
        curriculum::table
            .inner_join(semester::table)
            .inner_join(major::table)
            .filter(curriculum::id.eq(curriculum_id))
            .select((
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, curriculum_id: i64) -> QueryResult<usize> {
        diesel::delete(curriculum::table.filter(curriculum::id.eq(curriculum_id))).execute(conn)
    }
}

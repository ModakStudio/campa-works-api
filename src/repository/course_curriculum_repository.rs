use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course_curriculum::{CourseCurriculum, NewCourseCurriculum},
        curriculum::Curriculum,
        enums::*,
        major::Major,
        master_course::MasterCourse,
        semester::Semester,
    },
    schema::{course_curriculum, curriculum, major, master_course, semester},
};

#[macro_export]
macro_rules! apply_course_curriculum_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_curriculum_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_curriculum::id.eq(course_curriculum_id));
        }

        query = crate::apply_master_course_query_filters!(query, $params);

        query = crate::apply_curriculum_query_filters!(query, $params);

        query
    }};
}

pub struct CourseCurriculumRepository;

impl CourseCurriculumRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course_curriculum: &NewCourseCurriculum,
    ) -> QueryResult<CourseCurriculum> {
        diesel::insert_into(course_curriculum::table)
            .values(new_course_curriculum)
            .returning(CourseCurriculum::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CourseCurriculum, MasterCourse, Curriculum, Semester, Major)>> {
        let mut query = course_curriculum::table
            .inner_join(master_course::table)
            .inner_join(
                curriculum::table
                    .inner_join(semester::table)
                    .inner_join(major::table),
            )
            .select((
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .into_boxed();

        query = apply_course_curriculum_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_curriculum_id: i64,
    ) -> QueryResult<(CourseCurriculum, MasterCourse, Curriculum, Semester, Major)> {
        course_curriculum::table
            .inner_join(master_course::table)
            .inner_join(
                curriculum::table
                    .inner_join(semester::table)
                    .inner_join(major::table),
            )
            .filter(course_curriculum::id.eq(course_curriculum_id))
            .select((
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .first(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_curriculum_id: i64) -> QueryResult<usize> {
        diesel::delete(
            course_curriculum::table.filter(course_curriculum::id.eq(course_curriculum_id)),
        )
        .execute(conn)
    }
}

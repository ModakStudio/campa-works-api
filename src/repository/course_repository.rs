use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        course::{Course, NewCourse, UpdateCourse},
        course_curriculum::CourseCurriculum,
        curriculum::Curriculum,
        enums::*,
        major::Major,
        master_course::MasterCourse,
        semester::Semester,
    },
    schema::{course, course_curriculum, curriculum, major, master_course, semester},
};

#[macro_export]
macro_rules! apply_course_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(course_id) = $params
            .get("course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::id.eq(course_id));
        }

        query = crate::apply_course_curriculum_query_filters!(query, $params);

        if let Some(course_description) = $params
            .get("course_description")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(course::course_description.ilike(format!("%{}%", course_description)));
        }
        if let Some(grade) = $params
            .get("grade")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::grade.eq(grade));
        }
        if let Some(credit) = $params
            .get("credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::credit.eq(credit));
        }
        if let Some(lecture) = $params
            .get("lecture")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::lecture.eq(lecture));
        }
        if let Some(practice) = $params
            .get("practice")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::practice.eq(practice));
        }
        if let Some(course_category) = $params
            .get("course_category")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::course_category.eq(CourseCategory::from(course_category)));
        }
        if let Some(language) = $params
            .get("language")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::language.eq(Language::from(language)));
        }
        if let Some(section_number) = $params
            .get("section_number")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::section_number.eq(section_number));
        }
        if let Some(capacity) = $params
            .get("capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::capacity.eq(capacity));
        }
        if let Some(participant) = $params
            .get("participant")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::participant.eq(participant));
        }

        query
    }};
}

pub struct CourseRepository;

impl CourseRepository {
    pub fn create(conn: &mut PgConnection, new_course: &NewCourse) -> QueryResult<Course> {
        diesel::insert_into(course::table)
            .values(new_course)
            .returning(Course::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<
        Vec<(
            Course,
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
        )>,
    > {
        let mut query = course::table
            .inner_join(
                course_curriculum::table
                    .inner_join(master_course::table)
                    .inner_join(
                        curriculum::table
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    ),
            )
            .select((
                Course::as_select(),
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .into_boxed();

        query = apply_course_query_filters!(query, params);

        query.load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_id: i64,
    ) -> QueryResult<(
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )> {
        course::table
            .inner_join(
                course_curriculum::table
                    .inner_join(master_course::table)
                    .inner_join(
                        curriculum::table
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    ),
            )
            .filter(course::id.eq(course_id))
            .select((
                Course::as_select(),
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .first(conn)
    }

    pub fn find_by_master_course_id(
        conn: &mut PgConnection,
        master_course_id: i64,
    ) -> QueryResult<
        Vec<(
            Course,
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
        )>,
    > {
        course::table
            .inner_join(
                course_curriculum::table
                    .inner_join(master_course::table)
                    .inner_join(
                        curriculum::table
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    ),
            )
            .filter(course_curriculum::master_course_id.eq(master_course_id))
            .select((
                Course::as_select(),
                CourseCurriculum::as_select(),
                MasterCourse::as_select(),
                Curriculum::as_select(),
                Semester::as_select(),
                Major::as_select(),
            ))
            .load(conn)
    }

    pub fn find_course_amount_by_semester_id_and_major_id(
        conn: &mut PgConnection,
        semester_id: i64,
        major_id: i64,
    ) -> QueryResult<i64> {
        course::table
            .inner_join(
                course_curriculum::table
                    .inner_join(master_course::table)
                    .inner_join(
                        curriculum::table
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    ),
            )
            .filter(semester::id.eq(semester_id))
            .filter(major::id.eq(major_id))
            .count()
            .get_result(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        update_course: &UpdateCourse,
    ) -> QueryResult<Course> {
        diesel::update(course::table.filter(course::id.eq(course_id)))
            .set(update_course)
            .returning(Course::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> QueryResult<usize> {
        diesel::delete(course::table.filter(course::id.eq(course_id))).execute(conn)
    }
}

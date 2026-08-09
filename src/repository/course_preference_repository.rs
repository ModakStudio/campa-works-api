use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        course_preference::{CoursePreference, NewCoursePreference, UpdateCoursePreference},
        enums::{CourseType, ProfessorPosition, ProfessorStatus, SemesterStatus, SemesterType},
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
    },
    schema::{course_preference, master_course, professor, semester, users},
};

pub struct CoursePreferenceRepository;

impl CoursePreferenceRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_course_preference: &NewCoursePreference,
    ) -> QueryResult<CoursePreference> {
        diesel::insert_into(course_preference::table)
            .values(new_course_preference)
            .returning(CoursePreference::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(CoursePreference, Semester, Professor, User, MasterCourse)>> {
        let mut query = course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .into_boxed();

        if let Some(course_preference_id) =
            params.get("id").and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::id.eq(course_preference_id));
        }

        if let Some(semester_id) = params
            .get("semester_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::semester_id.eq(semester_id));
        }
        if let Some(semester_year) = params
            .get("semester_year")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(semester::year.eq(semester_year));
        }
        if let Some(semester_semester_) = params
            .get("semester_semester_")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::semester_.eq(SemesterType::from(semester_semester_)));
        }
        if let Some(semester_status) = params
            .get("semester_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(semester::status.eq(SemesterStatus::from(semester_status)));
        }

        if let Some(professor_id) = params
            .get("professor_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::professor_id.eq(professor_id));
        }
        if let Some(position) = params
            .get("position")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::position.eq(ProfessorPosition::from(position)));
        }
        if let Some(office) = params
            .get("office")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::office.ilike(format!("%{}%", office)));
        }
        if let Some(tel) = params
            .get("tel")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::tel.ilike(format!("%{}%", tel)));
        }
        if let Some(research_field) = params
            .get("research_field")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::research_field.ilike(format!("%{}%", research_field)));
        }
        if let Some(status) = params
            .get("status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::status.eq(ProfessorStatus::from(status)));
        }

        if let Some(user_id) = params
            .get("user_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor::user_id.eq(user_id));
        }
        if let Some(user_email) = params
            .get("user_email")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::email.eq(user_email));
        }
        if let Some(user_name) = params
            .get("user_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(users::name.ilike(format!("%{}%", user_name)));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_preference::master_course_id.eq(master_course_id));
        }
        if let Some(master_course_course_code) = params
            .get("master_course_course_code")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(
                master_course::course_code.ilike(format!("%{}%", master_course_course_code)),
            );
        }
        if let Some(master_course_name) = params
            .get("master_course_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(master_course::name.ilike(format!("%{}%", master_course_name)));
        }
        if let Some(master_course_credit) = params
            .get("master_course_credit")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::credit.eq(master_course_credit));
        }
        if let Some(master_course_lecture) = params
            .get("master_course_lecture")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::lecture.eq(master_course_lecture));
        }
        if let Some(master_course_practice) = params
            .get("master_course_practice")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(master_course::practice.eq(master_course_practice));
        }
        if let Some(master_course_course_type) = params
            .get("master_course_course_type")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(master_course::course_type.eq(CourseType::from(master_course_course_type)));
        }
        if let Some(master_course_is_core) = params
            .get("master_course_is_core")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(master_course::is_core.eq(master_course_is_core));
        }

        if let Some(priority) = params
            .get("priority")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course_preference::priority.eq(priority));
        }

        query
            .select((
                CoursePreference::as_select(),
                Semester::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        course_preference_id: i64,
    ) -> QueryResult<(CoursePreference, Semester, Professor, User, MasterCourse)> {
        course_preference::table
            .inner_join(semester::table)
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(master_course::table)
            .filter(course_preference::id.eq(course_preference_id))
            .select((
                CoursePreference::as_select(),
                Semester::as_select(),
                Professor::as_select(),
                User::as_select(),
                MasterCourse::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        course_preference_id: i64,
        update_course_preference: &UpdateCoursePreference,
    ) -> QueryResult<CoursePreference> {
        diesel::update(
            course_preference::table.filter(course_preference::id.eq(course_preference_id)),
        )
        .set(update_course_preference)
        .returning(CoursePreference::as_returning())
        .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, course_preference_id: i64) -> QueryResult<usize> {
        diesel::delete(
            course_preference::table.filter(course_preference::id.eq(course_preference_id)),
        )
        .execute(conn)
    }
}

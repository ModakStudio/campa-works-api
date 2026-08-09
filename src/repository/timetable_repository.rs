use std::collections::HashMap;

use diesel::prelude::*;

use crate::{
    models::{
        classroom::Classroom,
        course::Course,
        course_assignment::CourseAssignment,
        enums::{
            CourseCategory, CourseType, Language, ProfessorPosition, ProfessorStatus,
            SemesterStatus, SemesterType,
        },
        major::Major,
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        timetable::{NewTimetable, Timetable, UpdateTimetable},
        user::User,
    },
    schema::{
        classroom, course, course_assignment, major, master_course, professor, semester, timetable,
        users,
    },
};

pub struct TimetableRepository;

impl TimetableRepository {
    pub fn create(conn: &mut PgConnection, new_timetable: &NewTimetable) -> QueryResult<Timetable> {
        diesel::insert_into(timetable::table)
            .values(new_timetable)
            .returning(Timetable::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<
        Vec<(
            Timetable,
            CourseAssignment,
            Course,
            MasterCourse,
            Semester,
            Major,
            Professor,
            User,
            Classroom,
        )>,
    > {
        let mut query = timetable::table
            .inner_join(
                course_assignment::table
                    .inner_join(
                        course::table
                            .inner_join(master_course::table)
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    )
                    .inner_join(professor::table.inner_join(users::table)),
            )
            .inner_join(classroom::table)
            .into_boxed();

        if let Some(timetable_id) = params.get("id").and_then(|value| value.parse::<i64>().ok()) {
            query = query.filter(timetable::id.eq(timetable_id));
        }

        if let Some(assignment_id) = params
            .get("assignment_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(timetable::assignment_id.eq(assignment_id));
        }
        if let Some(course_id) = params
            .get("course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_assignment::course_id.eq(course_id));
        }
        if let Some(course_description) = params
            .get("course_description")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::description.ilike(format!("%{}%", course_description)));
        }
        if let Some(course_course_category) = params
            .get("course_course_category")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(course::course_category.eq(CourseCategory::from(course_course_category)));
        }
        if let Some(course_language) = params
            .get("course_language")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(course::language.eq(Language::from(course_language)));
        }
        if let Some(course_section_number) = params
            .get("course_section_number")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::section_number.eq(course_section_number));
        }
        if let Some(course_grade) = params
            .get("course_grade")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::grade.eq(course_grade));
        }
        if let Some(course_capacity) = params
            .get("course_capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(course::capacity.eq(course_capacity));
        }

        if let Some(master_course_id) = params
            .get("master_course_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::master_course_id.eq(master_course_id));
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

        if let Some(semester_id) = params
            .get("semester_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::semester_id.eq(semester_id));
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

        if let Some(major_id) = params
            .get("major_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course::major_id.eq(major_id));
        }
        if let Some(major_name) = params
            .get("major_name")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(major::name.ilike(format!("%{}%", major_name)));
        }

        if let Some(professor_id) = params
            .get("professor_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(course_assignment::professor_id.eq(professor_id));
        }
        if let Some(professor_position) = params
            .get("professor_position")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query =
                query.filter(professor::position.eq(ProfessorPosition::from(professor_position)));
        }
        if let Some(professor_office) = params
            .get("professor_office")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::office.ilike(format!("%{}%", professor_office)));
        }
        if let Some(professor_tel) = params
            .get("professor_tel")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::tel.ilike(format!("%{}%", professor_tel)));
        }
        if let Some(professor_research_field) = params
            .get("professor_research_field")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query
                .filter(professor::research_field.ilike(format!("%{}%", professor_research_field)));
        }
        if let Some(professor_status) = params
            .get("professor_status")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor::status.eq(ProfessorStatus::from(professor_status)));
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

        if let Some(classroom_id) = params
            .get("classroom_id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(timetable::classroom_id.eq(classroom_id));
        }
        if let Some(classroom_building) = params
            .get("classroom_building")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(classroom::building.ilike(format!("%{}%", classroom_building)));
        }
        if let Some(classroom_room) = params
            .get("classroom_room")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(classroom::room.ilike(format!("%{}%", classroom_room)));
        }
        if let Some(classroom_capacity) = params
            .get("classroom_capacity")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(classroom::capacity.eq(classroom_capacity));
        }
        if let Some(classroom_is_available) = params
            .get("classroom_is_available")
            .and_then(|value| value.parse::<bool>().ok())
        {
            query = query.filter(classroom::is_available.eq(classroom_is_available));
        }

        if let Some(day_of_week) = params
            .get("day_of_week")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(timetable::day_of_week.eq(day_of_week));
        }
        if let Some(start_period) = params
            .get("start_period")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(timetable::start_period.eq(start_period));
        }
        if let Some(end_period) = params
            .get("end_period")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(timetable::end_period.eq(end_period));
        }

        query
            .select((
                Timetable::as_select(),
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
                Classroom::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        timetable_id: i64,
    ) -> QueryResult<(
        Timetable,
        CourseAssignment,
        Course,
        MasterCourse,
        Semester,
        Major,
        Professor,
        User,
        Classroom,
    )> {
        timetable::table
            .inner_join(
                course_assignment::table
                    .inner_join(
                        course::table
                            .inner_join(master_course::table)
                            .inner_join(semester::table)
                            .inner_join(major::table),
                    )
                    .inner_join(professor::table.inner_join(users::table)),
            )
            .inner_join(classroom::table)
            .filter(timetable::id.eq(timetable_id))
            .select((
                Timetable::as_select(),
                CourseAssignment::as_select(),
                Course::as_select(),
                MasterCourse::as_select(),
                Semester::as_select(),
                Major::as_select(),
                Professor::as_select(),
                User::as_select(),
                Classroom::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        timetable_id: i64,
        update_timetable: &UpdateTimetable,
    ) -> QueryResult<Timetable> {
        diesel::update(timetable::table.filter(timetable::id.eq(timetable_id)))
            .set(update_timetable)
            .returning(Timetable::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, timetable_id: i64) -> QueryResult<usize> {
        diesel::delete(timetable::table.filter(timetable::id.eq(timetable_id))).execute(conn)
    }
}

use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::course::{CourseResponse, CreateCourseRequest, UpdateCourseRequest},
    error::app_error::AppError,
    models::course::{NewCourse, UpdateCourse},
    repository::{
        course_curriculum_repository::CourseCurriculumRepository,
        course_repository::CourseRepository, curriculum_repository::CurriculumRepository,
        major_repository::MajorRepository, semester_repository::SemesterRepository,
    },
};

pub struct CourseService;

impl CourseService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateCourseRequest,
    ) -> Result<CourseResponse, AppError> {
        let query_params = HashMap::from([
            (
                "course_curriculum_id".to_string(),
                request.course_curriculum_id.to_string(),
            ),
            (
                "section_number".to_string(),
                request.section_number.to_string(),
            ),
        ]);

        if !CourseRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::CourseAlreadyExists);
        }

        let new_course = NewCourse {
            course_curriculum_id: request.course_curriculum_id,

            course_description: request.course_description,

            grade: request.grade,
            credit: request.credit,
            lecture: request.lecture,
            practice: request.practice,

            course_category: request.course_category,

            language: request.language,

            section_number: request.section_number,
            capacity: request.capacity,
            participant: request.participant,
        };

        CourseRepository::create(conn, &new_course).map_err(|_| AppError::DatabaseError)?;

        let course = CourseRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(course.into())
    }

    pub fn create_all_in_new_semester(
        conn: &mut PgConnection,
        semester_id: i64,
    ) -> Result<Vec<CourseResponse>, AppError> {
        for major in
            MajorRepository::find_all(conn, &HashMap::new()).map_err(|_| AppError::DatabaseError)?
        {
            let query_parmas = HashMap::from([
                ("semester_id".to_string(), semester_id.to_string()),
                ("major_id".to_string(), major.id.to_string()),
            ]);
            let current_course_curriculum =
                CourseCurriculumRepository::find_all(conn, &query_parmas)
                    .map_err(|_| AppError::DatabaseError)?
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| unreachable!());

            for grade in 1..=4 {
                let check_semester =
                    SemesterRepository::find_previous_year_semester(conn, semester_id, grade - 1)
                        .map_err(|_| AppError::SemesterNotFound)?;

                let last_curriculum = CurriculumRepository::find_by_semester_id_and_major_id(
                    conn,
                    check_semester.id,
                    major.id,
                )
                .map_err(|_| AppError::CurriculumNotFound)?;

                for course_curriculum in
                    CourseCurriculumRepository::find_by_curriculum_id(conn, last_curriculum.0.id)
                        .map_err(|_| AppError::DatabaseError)?
                {
                    let sample_course = CourseRepository::find_by_master_course_id(
                        conn,
                        course_curriculum.0.master_course_id,
                    )
                    .map_err(|_| AppError::DatabaseError)?
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| unreachable!());

                    for count in 1
                        ..=CourseRepository::find_course_amount_by_semester_id_and_major_id(
                            conn,
                            check_semester.id,
                            major.id,
                        )
                        .map_err(|_| AppError::DatabaseError)?
                    {
                        let new_course = CreateCourseRequest {
                            course_curriculum_id: current_course_curriculum.0.id,

                            course_description: None,

                            grade: grade,
                            credit: sample_course.0.credit,
                            lecture: sample_course.0.lecture,
                            practice: sample_course.0.practice,

                            course_category: sample_course.0.course_category,

                            language: sample_course.0.language,
                            section_number: count as i32,
                            capacity: sample_course.0.capacity,
                            participant: 0,
                        };

                        CourseService::create(conn, new_course)?;
                    }
                }
            }
        }
        Ok(Vec::new())
    }

    pub fn get_by_id(conn: &mut PgConnection, course_id: i64) -> Result<CourseResponse, AppError> {
        let course =
            CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        Ok(course.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<CourseResponse>, AppError> {
        let courses =
            CourseRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(courses.into_iter().map(Into::into).collect())
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        request: UpdateCourseRequest,
    ) -> Result<CourseResponse, AppError> {
        CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        let update_course = UpdateCourse {
            course_description: request.course_description,

            grade: request.grade,
            credit: request.credit,
            lecture: request.lecture,
            practice: request.practice,

            course_category: request.course_category,

            language: request.language,

            section_number: request.section_number,
            capacity: request.capacity,
            participant: request.participant,
        };

        CourseRepository::update(conn, course_id, &update_course)
            .map_err(|_| AppError::DatabaseError)?;

        let course =
            CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::DatabaseError)?;

        Ok(course.into())
    }

    pub fn delete(conn: &mut PgConnection, course_id: i64) -> Result<(), AppError> {
        CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        CourseRepository::delete(conn, course_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}

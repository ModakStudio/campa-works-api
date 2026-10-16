use std::collections::HashMap;

use diesel::PgConnection;

use crate::{
    dto::course::{CourseResponse, CreateCourseRequest, UpdateCourseRequest},
    error::app_error::AppError,
    models::{
        course::{NewCourse, UpdateCourse},
        enums::{CourseCategory, Language},
    },
    repository::{
        course_curriculum_repository::CourseCurriculumRepository,
        course_repository::CourseRepository,
    },
    service::curriculum_service::CurriculumService,
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

    pub fn create_auto(
        conn: &mut PgConnection,
        semester_id: i64,
    ) -> Result<Vec<CourseResponse>, AppError> {
        let mut courses = Vec::new();

        for last_curriculum in
            CurriculumService::get_curriculums_for_all_grades_from_semester_id(conn, semester_id)?
        {
            for course_curriculum in
                CourseCurriculumRepository::find_by_curriculum_id(conn, last_curriculum.id)
                    .map_err(|_| AppError::DatabaseError)?
            {
                let new_course = NewCourse {
                    course_curriculum_id: course_curriculum.0.id,

                    course_description: None,

                    course_category: CourseCategory::GeneralElective,

                    language: Language::Korean,

                    section_number:
                        CourseRepository::find_course_amount_by_semester_id_and_master_course_id(
                            conn,
                            semester_id,
                            course_curriculum.1.id,
                        )
                        .map_err(|_| AppError::DatabaseError)
                        .unwrap_or(0) as i32
                            + 1,
                    capacity: if course_curriculum.1.lecture > course_curriculum.1.practice {
                        60
                    } else {
                        70
                    },
                    participant: 0,
                };

                let created_course = CourseRepository::create(conn, &new_course)
                    .map_err(|_| AppError::DatabaseError)?;
                courses.push(
                    (
                        created_course,
                        course_curriculum.0,
                        course_curriculum.1,
                        course_curriculum.2,
                        course_curriculum.3,
                        course_curriculum.4,
                    )
                        .into(),
                );
            }
        }

        Ok(courses)
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

    pub fn get_offered_courses_by_semester_id(
        conn: &mut PgConnection,
        semester_id: i64,
    ) -> Result<Vec<CourseResponse>, AppError> {
        Ok(
            CurriculumService::get_curriculums_for_all_grades_from_semester_id(conn, semester_id)?
                .into_iter()
                .flat_map(|curriculum| {
                    CourseRepository::find_by_curriculum_id(conn, curriculum.id)
                        .map_err(|_| AppError::DatabaseError)
                        .unwrap_or_else(|_| Vec::new())
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<CourseResponse>>()
                })
                .collect::<Vec<CourseResponse>>(),
        )
    }

    pub fn update(
        conn: &mut PgConnection,
        course_id: i64,
        request: UpdateCourseRequest,
    ) -> Result<CourseResponse, AppError> {
        CourseRepository::find_by_id(conn, course_id).map_err(|_| AppError::CourseNotFound)?;

        let update_course = UpdateCourse {
            course_description: request.course_description,

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

use diesel::PgConnection;

use crate::{
    error::app_error::AppError,
    models::{
        course::Course, course_assignment::NewCourseAssignment,
        course_curriculum::CourseCurriculum, curriculum::Curriculum, enums::QuotaType,
        major::Major, master_course::MasterCourse, professor::Professor, semester::Semester,
        user::User,
    },
    repository::{
        course_assignment_repository::CourseAssignmentRepository,
        course_curriculum_repository::CourseCurriculumRepository,
        course_pool_repository::CoursePoolRepository,
        course_preference_repository::CoursePreferenceRepository,
        professor_quota_repository::ProfessorQuotaRepository,
    },
};

pub fn init_hungarian_matrix(
    conn: &mut PgConnection,
    courses: &Vec<(
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )>,
    professors: &Vec<(Professor, User, Semester)>,
    semester_id: i64,
) -> Result<Vec<Vec<i32>>, AppError> {
    let mut hungarian_matrix = vec![vec![0; courses.len()]; professors.len()];
    for (row_idx, (professor, _, _)) in professors.iter().enumerate() {
        for (col_idx, (course, _, _, _, _, _)) in courses.iter().enumerate() {
            let master_course_id =
                CourseCurriculumRepository::find_by_id(conn, course.course_curriculum_id)
                    .map_err(|_| AppError::DatabaseError)?
                    .1
                    .id;

            hungarian_matrix[row_idx][col_idx] = match CoursePreferenceRepository::find_by_professor_id_and_semester_id_and_master_course_id(
                conn,
                professor.id,
                semester_id,
                master_course_id,
            )
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            {
                Some((course_preference, _, _, _, _)) => course_preference.priority,
                None => {
                    if CoursePoolRepository::find_by_professor_id_and_master_course_id(
                        conn,
                        professor.id,
                        master_course_id,
                    )
                    .unwrap()
                    .is_empty()
                    {
                        9
                    } else {
                        8
                    }
                }
            };
        }
    }

    Ok(hungarian_matrix)
}

fn minimize_hungarian_matrix(
    hungarian_matrix: &mut Vec<Vec<i32>>,
    courses_len: usize,
    professors_len: usize,
) {
    if courses_len == 0 || professors_len == 0 {
        return;
    }

    for row in hungarian_matrix.iter_mut().take(professors_len) {
        let min_value = *row.iter().take(courses_len).min().unwrap();
        for value in row.iter_mut().take(courses_len) {
            *value -= min_value;
        }
    }
    for col_idx in 0..courses_len {
        let min_value = hungarian_matrix
            .iter()
            .map(|row| row[col_idx])
            .min()
            .unwrap();
        for row_idx in 0..professors_len {
            hungarian_matrix[row_idx][col_idx] -= min_value;
        }
    }
}

fn get_zero_idx_vec(
    hungarian_matrix: &Vec<Vec<i32>>,
    professors_len: usize,
    col_idx: usize,
) -> Vec<usize> {
    hungarian_matrix
        .iter()
        .take(professors_len)
        .enumerate()
        .filter_map(|(row_idx, row)| (row[col_idx] == 0).then_some(row_idx))
        .collect()
}

fn get_remaining_professor_quota(
    conn: &mut PgConnection,
    professor_id: i64,
    semester_id: i64,
) -> Result<i32, AppError> {
    let professor_quota = ProfessorQuotaRepository::find_by_professor_id_and_semester_id(
        conn,
        professor_id,
        semester_id,
    )
    .map_err(|_| AppError::DatabaseError)?;

    let quota_type = professor_quota.0.quota_type;

    Ok((professor_quota.0.quota_value
        - CourseAssignmentRepository::find_by_professor_id_and_semester_id(
            conn,
            professor_id,
            semester_id,
        )
        .map_err(|_| AppError::DatabaseError)?
        .iter()
        .map(|(_, course, _, _, _, _, _, _, _)| match quota_type {
            QuotaType::Credit => course.credit,
            QuotaType::Hour => course.lecture + course.practice,
        })
        .sum::<i32>())
        * match quota_type {
            QuotaType::Credit => 1,
            QuotaType::Hour => 3,
        })
}

fn execute_shallow(
    conn: &mut PgConnection,
    courses: &mut Vec<(
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )>,
    professors: &mut Vec<(Professor, User, Semester)>,
    hungarian_matrix: &mut Vec<Vec<i32>>,
    semester_id: i64,
) -> Result<bool, AppError> {
    let mut is_changed = false;

    let mut col_idx = 0;
    while col_idx < courses.len() {
        let zero_idx_vec = get_zero_idx_vec(hungarian_matrix, professors.len(), col_idx);
        if zero_idx_vec.len() == 1 {
            let row_idx = zero_idx_vec[0];

            CourseAssignmentRepository::create(
                conn,
                &NewCourseAssignment {
                    course_id: courses[col_idx].0.id,
                    professor_id: professors[row_idx].0.id,
                },
            )
            .map_err(|_| AppError::DatabaseError)?;

            if get_remaining_professor_quota(conn, professors[row_idx].0.id, semester_id)? <= 0 {
                professors.remove(row_idx);
                hungarian_matrix.remove(row_idx);
            }
            courses.remove(col_idx);
            for row in hungarian_matrix.iter_mut() {
                row.remove(col_idx);
            }

            is_changed = true;
        } else {
            col_idx += 1;
        }
    }

    Ok(is_changed)
}

fn execute_deep(
    conn: &mut PgConnection,
    courses: &mut Vec<(
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )>,
    professors: &mut Vec<(Professor, User, Semester)>,
    hungarian_matrix: &mut Vec<Vec<i32>>,
    semester_id: i64,
) -> Result<bool, AppError> {
    // Resolve the most constrained ambiguous course first. When courses have
    // the same number of zero-cost professors, prefer a professor with fewer
    // other zero-cost course options so their alternatives remain available.
    let candidate = (0..courses.len())
        .filter_map(|col_idx| {
            let zero_idx_vec = get_zero_idx_vec(hungarian_matrix, professors.len(), col_idx);
            (zero_idx_vec.len() > 1).then_some((col_idx, zero_idx_vec))
        })
        .min_by_key(|(_, zero_idx_vec)| zero_idx_vec.len());

    let Some((col_idx, zero_idx_vec)) = candidate else {
        return Ok(false);
    };

    let row_idx = zero_idx_vec
        .into_iter()
        .min_by_key(|&row_idx| {
            hungarian_matrix[row_idx]
                .iter()
                .filter(|&&cost| cost == 0)
                .count()
        })
        .expect("an ambiguous course must have at least two zero-cost professors");

    CourseAssignmentRepository::create(
        conn,
        &NewCourseAssignment {
            course_id: courses[col_idx].0.id,
            professor_id: professors[row_idx].0.id,
        },
    )
    .map_err(|_| AppError::DatabaseError)?;

    if get_remaining_professor_quota(conn, professors[row_idx].0.id, semester_id)? <= 0 {
        professors.remove(row_idx);
        hungarian_matrix.remove(row_idx);
    }

    courses.remove(col_idx);
    for row in hungarian_matrix.iter_mut() {
        row.remove(col_idx);
    }

    Ok(true)
}

pub fn execute_round(
    conn: &mut PgConnection,
    courses: &mut Vec<(
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )>,
    professors: &mut Vec<(Professor, User, Semester)>,
    hungarian_matrix: &mut Vec<Vec<i32>>,
    semester_id: i64,
) -> Result<(), AppError> {
    let courses_len = courses.len();
    let professors_len = professors.len();

    if courses_len == 0 || professors_len == 0 {
        return Ok(());
    }

    minimize_hungarian_matrix(hungarian_matrix, courses_len, professors_len);

    loop {
        while execute_shallow(conn, courses, professors, hungarian_matrix, semester_id)? {
            // Keep resolving all currently unambiguous columns before choosing
            // among ambiguous zero-cost professors.
        }
        if !execute_deep(conn, courses, professors, hungarian_matrix, semester_id)? {
            break;
        }
    }

    Ok(())
}

pub fn execute_until_exhausted(
    conn: &mut PgConnection,
    courses: &mut Vec<(
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )>,
    professors: &mut Vec<(Professor, User, Semester)>,
    hungarian_matrix: &mut Vec<Vec<i32>>,
    semester_id: i64,
) -> Result<(), AppError> {
    while !courses.is_empty() && !professors.is_empty() {
        let courses_before = courses.len();
        let professors_before = professors.len();

        execute_round(conn, courses, professors, hungarian_matrix, semester_id)?;

        if courses.len() == courses_before && professors.len() == professors_before {
            break;
        }
    }

    Ok(())
}

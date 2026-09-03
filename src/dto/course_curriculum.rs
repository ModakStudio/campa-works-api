use serde::{Deserialize, Serialize};

use crate::{
    dto::{curriculum::CurriculumResponse, master_course::MasterCourseResponse},
    models::{
        course_curriculum::CourseCurriculum, curriculum::Curriculum, major::Major,
        master_course::MasterCourse, semester::Semester,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCourseCurriculumRequest {
    pub master_course_id: i64,
    pub curriculum_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseCurriculumResponse {
    pub id: i64,

    pub master_course_id: MasterCourseResponse,
    pub curriculum: CurriculumResponse,
}

impl From<(CourseCurriculum, MasterCourse, Curriculum, Semester, Major)>
    for CourseCurriculumResponse
{
    fn from(
        (course_curriculum, master_course, curriculum, semester, major): (
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
        ),
    ) -> Self {
        Self {
            id: course_curriculum.id,

            master_course_id: MasterCourseResponse::from(master_course),
            curriculum: CurriculumResponse::from((curriculum, semester, major)),
        }
    }
}

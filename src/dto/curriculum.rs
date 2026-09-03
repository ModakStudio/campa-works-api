use serde::{Deserialize, Serialize};

use crate::{
    dto::{major::MajorResponse, semester::SemesterResponse},
    models::{curriculum::Curriculum, major::Major, semester::Semester},
};

#[derive(Debug, Deserialize)]
pub struct CreateCurriculumRequest {
    pub semester_id: i64,
    pub major_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CurriculumResponse {
    pub id: i64,

    pub semester: SemesterResponse,
    pub major: MajorResponse,
}

impl From<(Curriculum, Semester, Major)> for CurriculumResponse {
    fn from((curriculum, semester, major): (Curriculum, Semester, Major)) -> Self {
        Self {
            id: curriculum.id,

            semester: SemesterResponse::from(semester),
            major: MajorResponse::from(major),
        }
    }
}

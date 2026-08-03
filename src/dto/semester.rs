use serde::{Deserialize, Serialize};

use crate::models::{
    enums::{SemesterStatus, SemesterType},
    semester::Semester,
};

#[derive(Debug, Deserialize)]
pub struct CreateSemesterRequest {
    pub year: i32,
    pub semester_: SemesterType,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSemesterRequest {
    pub year: Option<i32>,
    pub semester_: Option<SemesterType>,
    pub status: Option<SemesterStatus>,
}

#[derive(Debug, Serialize)]
pub struct SemesterResponse {
    pub id: i64,

    pub year: i32,
    pub semester_: SemesterType,

    pub status: SemesterStatus,
}

impl From<Semester> for SemesterResponse {
    fn from(semester: Semester) -> Self {
        Self {
            id: semester.id,
            year: semester.year,
            semester_: semester.semester_,
            status: semester.status,
        }
    }
}

use std::fmt::Display;

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

    pub semester_status: Option<SemesterStatus>,
}

#[derive(Debug, Serialize)]
pub struct SemesterResponse {
    pub id: i64,

    pub year: i32,
    pub semester_: SemesterType,

    pub semester_status: SemesterStatus,
}

impl From<Semester> for SemesterResponse {
    fn from(semester: Semester) -> Self {
        Self {
            id: semester.id,

            year: semester.year,
            semester_: semester.semester_,

            semester_status: semester.semester_status,
        }
    }
}

impl From<&str> for SemesterType {
    fn from(s: &str) -> Self {
        match s {
            "FIRST" => SemesterType::First,
            "SUMMER" => SemesterType::Summer,
            "SECOND" => SemesterType::Second,
            "WINTER" => SemesterType::Winter,
            _ => panic!("Invalid semester type"),
        }
    }
}

impl Display for SemesterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SemesterType::First => "FIRST",
            SemesterType::Summer => "SUMMER",
            SemesterType::Second => "SECOND",
            SemesterType::Winter => "WINTER",
        };
        write!(f, "{}", s)
    }
}

impl From<&str> for SemesterStatus {
    fn from(s: &str) -> Self {
        match s {
            "OPEN" => SemesterStatus::Open,
            "CLOSED" => SemesterStatus::Closed,
            _ => panic!("Invalid semester status"),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    dto::{professor::ProfessorBriefResponse, semester::SemesterResponse},
    models::{
        enums::QuotaType, professor::Professor, professor_quota::ProfessorQuota,
        semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateProfessorQuotaRequest {
    pub professor_id: i64,
    pub semester_id: i64,

    pub quota_type: QuotaType,
    pub quota_value: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfessorQuotaRequest {
    pub quota_type: Option<QuotaType>,
    pub quota_value: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProfessorQuotaResponse {
    pub id: i64,

    pub professor: ProfessorBriefResponse,
    pub semester: SemesterResponse,

    pub quota_type: QuotaType,
    pub quota_value: i32,
}

impl From<(ProfessorQuota, Professor, User, Semester)> for ProfessorQuotaResponse {
    fn from(
        (professor_quota, professor, user, semester): (ProfessorQuota, Professor, User, Semester),
    ) -> Self {
        Self {
            id: professor_quota.id,

            professor: ProfessorBriefResponse::from((professor, user)),
            semester: SemesterResponse::from(semester),

            quota_type: professor_quota.quota_type,
            quota_value: professor_quota.quota_value,
        }
    }
}

impl From<&str> for QuotaType {
    fn from(s: &str) -> Self {
        match s {
            "CREDIT" => QuotaType::Credit,
            "HOUR" => QuotaType::Hour,
            _ => QuotaType::Hour, // Default value
        }
    }
}

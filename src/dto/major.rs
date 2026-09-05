use serde::{Deserialize, Serialize};

use crate::models::{enums::MajorStatus, major::Major};

#[derive(Debug, Deserialize)]
pub struct CreateMajorRequest {
    pub major_name: String,
    pub major_code: String,

    pub major_status: MajorStatus,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMajorRequest {
    pub major_name: Option<String>,
    pub major_code: Option<String>,

    pub major_status: Option<MajorStatus>,
}

#[derive(Debug, Serialize)]
pub struct MajorResponse {
    pub id: i64,

    pub major_name: String,
    pub major_code: String,

    pub major_status: MajorStatus,
}

impl From<Major> for MajorResponse {
    fn from(major: Major) -> Self {
        Self {
            id: major.id,

            major_name: major.major_name,
            major_code: major.major_code,

            major_status: major.major_status,
        }
    }
}

impl From<&str> for MajorStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => MajorStatus::Active,
            "INACTIVE" => MajorStatus::Inactive,
            _ => MajorStatus::Inactive,
        }
    }
}

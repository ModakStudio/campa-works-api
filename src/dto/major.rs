use serde::{Deserialize, Serialize};

use crate::models::major::Major;

#[derive(Debug, Deserialize)]
pub struct CreateMajorRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMajorRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MajorResponse {
    pub id: i64,
    pub name: String,
}

impl From<Major> for MajorResponse {
    fn from(major: Major) -> Self {
        Self {
            id: major.id,
            name: major.name,
        }
    }
}

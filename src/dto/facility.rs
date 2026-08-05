use serde::{Deserialize, Serialize};

use crate::models::facility::Facility;

#[derive(Debug, Deserialize)]
pub struct CreateFacilityRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFacilityRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FacilityResponse {
    pub id: i64,

    pub name: String,
    pub description: Option<String>,
}

impl From<Facility> for FacilityResponse {
    fn from(facility: Facility) -> Self {
        Self {
            id: facility.id,

            name: facility.name,
            description: facility.description,
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::models::facility::Facility;

#[derive(Debug, Deserialize)]
pub struct CreateFacilityRequest {
    pub facility_name: String,
    pub facility_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFacilityRequest {
    pub facility_name: Option<String>,
    pub facility_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FacilityResponse {
    pub id: i64,

    pub facility_name: String,
    pub facility_description: Option<String>,
}

impl From<Facility> for FacilityResponse {
    fn from(facility: Facility) -> Self {
        Self {
            id: facility.id,

            facility_name: facility.facility_name,
            facility_description: facility.facility_description,
        }
    }
}

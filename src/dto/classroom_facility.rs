use serde::{Deserialize, Serialize};

use crate::{
    dto::{classroom::ClassroomResponse, facility::FacilityResponse},
    models::{classroom::Classroom, classroom_facility::ClassroomFacility, facility::Facility},
};

#[derive(Debug, Deserialize)]
pub struct CreateClassroomFacilityRequest {
    pub classroom_id: i64,
    pub facility_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ClassroomFacilityResponse {
    pub id: i64,

    pub classroom: ClassroomResponse,
    pub facility: FacilityResponse,
}

impl From<(ClassroomFacility, Classroom, Facility)> for ClassroomFacilityResponse {
    fn from((classroom_facility, classroom, facility): (ClassroomFacility, Classroom, Facility)) -> Self {
        Self {
            id: classroom_facility.id,

            classroom: ClassroomResponse::from(classroom),
            facility: FacilityResponse::from(facility),
        }
    }
}
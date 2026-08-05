use serde::{Deserialize, Serialize};

use crate::{
    dto::{facility::FacilityResponse, master_course::MasterCourseResponse},
    models::{course_facility::CourseFacility, facility::Facility, master_course::MasterCourse},
};

#[derive(Debug, Deserialize)]
pub struct CreateCourseFacilityRequest {
    pub master_course_id: i64,
    pub facility_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseFacilityResponse {
    pub id: i64,

    pub master_course: MasterCourseResponse,
    pub facility: FacilityResponse,
}

impl From<(CourseFacility, MasterCourse, Facility)> for CourseFacilityResponse {
    fn from(
        (course_facility, master_course, facility): (CourseFacility, MasterCourse, Facility),
    ) -> Self {
        Self {
            id: course_facility.id,

            master_course: MasterCourseResponse::from(master_course),
            facility: FacilityResponse::from(facility),
        }
    }
}

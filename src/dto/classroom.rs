use serde::{Deserialize, Serialize};

use crate::models::classroom::Classroom;

#[derive(Debug, Deserialize)]
pub struct CreateClassroomRequest {
    pub building: String,
    pub room: String,

    pub capacity: i32,
    pub is_available: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClassroomRequest {
    pub building: Option<String>,
    pub room: Option<String>,

    pub capacity: Option<i32>,
    pub is_available: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ClassroomResponse {
    pub id: i64,

    pub building: String,
    pub room: String,

    pub capacity: i32,
    pub is_available: bool,
}

impl From<Classroom> for ClassroomResponse {
    fn from(classroom: Classroom) -> Self {
        Self {
            id: classroom.id,

            building: classroom.building,
            room: classroom.room,

            capacity: classroom.capacity,
            is_available: classroom.is_available,
        }
    }
}

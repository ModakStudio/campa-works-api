use serde::{Deserialize, Serialize};

use crate::models::{
    enums::{ProfessorPosition, ProfessorStatus},
    professor::Professor,
};

#[derive(Debug, Deserialize)]
pub struct CreateProfessorRequest {
    pub user_id: i64,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfessorRequest {
    pub position: Option<ProfessorPosition>,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: Option<ProfessorStatus>,
}

#[derive(Debug, Serialize)]
pub struct ProfessorResponse {
    pub id: i64,

    pub user_id: i64,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: ProfessorStatus,
}

impl From<Professor> for ProfessorResponse {
    fn from(professor: Professor) -> Self {
        Self {
            id: professor.id,
            user_id: professor.user_id,

            position: professor.position,

            office: professor.office,
            tel: professor.tel,
            research_field: professor.research_field,

            status: professor.status,
        }
    }
}

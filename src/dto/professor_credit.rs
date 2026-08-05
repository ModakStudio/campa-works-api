use serde::{Deserialize, Serialize};

use crate::{
    dto::{professor::ProfessorResponse, semester::SemesterResponse},
    models::{
        professor::Professor, professor_credit::ProfessorCredit, semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateProfessorCreditRequest {
    pub professor_id: i64,
    pub semester_id: i64,

    pub target_credit: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfessorCreditRequest {
    pub target_credit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProfessorCreditResponse {
    pub id: i64,

    pub professor: ProfessorResponse,
    pub semester: SemesterResponse,

    pub target_credit: i32,
}

impl From<(ProfessorCredit, Professor, User, Semester)> for ProfessorCreditResponse {
    fn from(
        (professor_credit, professor, user, semester): (ProfessorCredit, Professor, User, Semester),
    ) -> Self {
        Self {
            id: professor_credit.id,

            professor: ProfessorResponse::from((professor, user)),
            semester: SemesterResponse::from(semester),

            target_credit: professor_credit.target_credit,
        }
    }
}

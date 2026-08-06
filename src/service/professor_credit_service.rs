use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    dto::professor_credit::{
        CreateProfessorCreditRequest, ProfessorCreditResponse, UpdateProfessorCreditRequest,
    },
    error::app_error::AppError,
    models::professor_credit::{NewProfessorCredit, UpdateProfessorCredit},
    repository::{
        professor_credit_repository::ProfessorCreditRepository,
        professor_repository::ProfessorRepository, semester_repository::SemesterRepository,
    },
};

pub struct ProfessorCreditService;

impl ProfessorCreditService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateProfessorCreditRequest,
    ) -> Result<ProfessorCreditResponse, AppError> {
        SemesterRepository::find_by_id(conn, request.semester_id)
            .map_err(|_| AppError::SemesterNotFound)?;

        ProfessorRepository::find_by_id(conn, request.professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        let query_params = HashMap::from([
            ("professor_id".to_string(), request.professor_id.to_string()),
            ("semester_id".to_string(), request.semester_id.to_string()),
        ]);

        if !ProfessorCreditRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::ProfessorCreditAlreadyExists);
        }

        let new_professor_credit = NewProfessorCredit {
            professor_id: request.professor_id,
            semester_id: request.semester_id,
            target_credit: request.target_credit,
        };

        ProfessorCreditRepository::create(conn, &new_professor_credit)
            .map_err(|_| AppError::DatabaseError)?;

        let professor_credit = ProfessorCreditRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(professor_credit.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<ProfessorCreditResponse>, AppError> {
        let professor_credits = ProfessorCreditRepository::find_all(conn, params)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(professor_credits.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        professor_credit_id: i64,
    ) -> Result<ProfessorCreditResponse, AppError> {
        let professor_credit = ProfessorCreditRepository::find_by_id(conn, professor_credit_id)
            .map_err(|_| AppError::ProfessorCreditNotFound)?;

        Ok(professor_credit.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_credit_id: i64,
        request: UpdateProfessorCreditRequest,
    ) -> Result<ProfessorCreditResponse, AppError> {
        ProfessorCreditRepository::find_by_id(conn, professor_credit_id)
            .map_err(|_| AppError::ProfessorCreditNotFound)?;

        let update_professor_credit = UpdateProfessorCredit {
            target_credit: request.target_credit,
        };

        ProfessorCreditRepository::update(conn, professor_credit_id, &update_professor_credit)
            .map_err(|_| AppError::DatabaseError)?;

        let professor_credit = ProfessorCreditRepository::find_by_id(conn, professor_credit_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(professor_credit.into())
    }

    pub fn delete(conn: &mut PgConnection, professor_credit_id: i64) -> Result<(), AppError> {
        ProfessorCreditRepository::find_by_id(conn, professor_credit_id)
            .map_err(|_| AppError::ProfessorCreditNotFound)?;

        ProfessorCreditRepository::delete(conn, professor_credit_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}

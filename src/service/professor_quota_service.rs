use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    dto::professor_quota::{
        CreateProfessorQuotaRequest, ProfessorQuotaResponse, UpdateProfessorQuotaRequest,
    },
    error::app_error::AppError,
    models::professor_quota::{NewProfessorQuota, UpdateProfessorQuota},
    repository::{
        professor_quota_repository::ProfessorQuotaRepository,
        professor_repository::ProfessorRepository, semester_repository::SemesterRepository,
    },
};

pub struct ProfessorQuotaService;

impl ProfessorQuotaService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateProfessorQuotaRequest,
    ) -> Result<ProfessorQuotaResponse, AppError> {
        SemesterRepository::find_by_id(conn, request.semester_id)
            .map_err(|_| AppError::SemesterNotFound)?;

        ProfessorRepository::find_by_id(conn, request.professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        let query_params = HashMap::from([
            ("professor_id".to_string(), request.professor_id.to_string()),
            ("semester_id".to_string(), request.semester_id.to_string()),
        ]);

        if !ProfessorQuotaRepository::find_all(conn, &query_params)
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            return Err(AppError::ProfessorQuotaAlreadyExists);
        }

        let new_professor_quota = NewProfessorQuota {
            professor_id: request.professor_id,
            semester_id: request.semester_id,
            quota_type: request.quota_type,
            quota_value: request.quota_value,
        };

        ProfessorQuotaRepository::create(conn, &new_professor_quota)
            .map_err(|_| AppError::DatabaseError)?;

        let professor_quota = ProfessorQuotaRepository::find_all(conn, &query_params)
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());

        Ok(professor_quota.into())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<ProfessorQuotaResponse>, AppError> {
        let professor_quotas = ProfessorQuotaRepository::find_all(conn, params)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(professor_quotas.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        professor_quota_id: i64,
    ) -> Result<ProfessorQuotaResponse, AppError> {
        let professor_quota = ProfessorQuotaRepository::find_by_id(conn, professor_quota_id)
            .map_err(|_| AppError::ProfessorQuotaNotFound)?;

        Ok(professor_quota.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_quota_id: i64,
        request: UpdateProfessorQuotaRequest,
    ) -> Result<ProfessorQuotaResponse, AppError> {
        ProfessorQuotaRepository::find_by_id(conn, professor_quota_id)
            .map_err(|_| AppError::ProfessorQuotaNotFound)?;

        let update_professor_quota = UpdateProfessorQuota {
            quota_type: request.quota_type,
            quota_value: request.quota_value,
        };

        ProfessorQuotaRepository::update(conn, professor_quota_id, &update_professor_quota)
            .map_err(|_| AppError::DatabaseError)?;

        let professor_quota = ProfessorQuotaRepository::find_by_id(conn, professor_quota_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(professor_quota.into())
    }

    pub fn delete(conn: &mut PgConnection, professor_quota_id: i64) -> Result<(), AppError> {
        ProfessorQuotaRepository::find_by_id(conn, professor_quota_id)
            .map_err(|_| AppError::ProfessorQuotaNotFound)?;

        ProfessorQuotaRepository::delete(conn, professor_quota_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}

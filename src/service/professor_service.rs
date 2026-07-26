use diesel::PgConnection;

use crate::{
    dto::professor::{CreateProfessorRequest, ProfessorResponse, UpdateProfessorRequest},
    error::app_error::AppError,
    models::{
        enums::ProfessorStatus,
        professor::{NewProfessor, UpdateProfessor},
    },
    repository::{professor_repository::ProfessorRepository, user_repository::UserRepository},
};

pub struct ProfessorService;

impl ProfessorService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateProfessorRequest,
    ) -> Result<ProfessorResponse, AppError> {
        UserRepository::find_by_id(conn, request.user_id).map_err(|_| AppError::UserNotFound)?;

        if ProfessorRepository::find_by_user_id(conn, request.user_id).is_ok() {
            return Err(AppError::ProfessorAlreadyExists);
        }

        let new_professor = NewProfessor {
            user_id: request.user_id,

            position: request.position,

            office: request.office,
            tel: request.tel,
            research_field: request.research_field,

            status: ProfessorStatus::Active,
        };

        ProfessorRepository::create(conn, &new_professor).map_err(|_| AppError::DatabaseError)?;

        let professor = ProfessorRepository::find_by_user_id(conn, request.user_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(professor.into())
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        professor_id: i64,
    ) -> Result<ProfessorResponse, AppError> {
        let professor = ProfessorRepository::find_by_id(conn, professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        Ok(professor.into())
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<ProfessorResponse>, AppError> {
        let professors =
            ProfessorRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?;

        Ok(professors.into_iter().map(Into::into).collect())
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_id: i64,
        request: UpdateProfessorRequest,
    ) -> Result<ProfessorResponse, AppError> {
        ProfessorRepository::find_by_id(conn, professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        let update_professor = UpdateProfessor {
            position: request.position,

            office: request.office,
            tel: request.tel,
            research_field: request.research_field,

            status: request.status,
        };

        ProfessorRepository::update(conn, professor_id, &update_professor)
            .map_err(|_| AppError::DatabaseError)?;

        let professor = ProfessorRepository::find_by_id(conn, professor_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(professor.into())
    }

    pub fn delete(conn: &mut PgConnection, professor_id: i64) -> Result<(), AppError> {
        ProfessorRepository::find_by_id(conn, professor_id)
            .map_err(|_| AppError::ProfessorNotFound)?;

        ProfessorRepository::delete(conn, professor_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}

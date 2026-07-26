// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "professor_position"))]
    pub struct ProfessorPosition;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "professor_status"))]
    pub struct ProfessorStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProfessorPosition;
    use super::sql_types::ProfessorStatus;

    professor (id) {
        id -> Int8,
        user_id -> Int8,
        position -> ProfessorPosition,
        #[max_length = 255]
        office -> Nullable<Varchar>,
        #[max_length = 50]
        tel -> Nullable<Varchar>,
        #[max_length = 255]
        research_field -> Nullable<Varchar>,
        status -> ProfessorStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int8,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        role -> UserRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(professor -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(professor, users,);

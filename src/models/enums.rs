use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

//
// User
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    #[db_rename = "ADMIN"]
    Admin,

    #[db_rename = "PROFESSOR"]
    Professor,

    #[db_rename = "ASSISTANT"]
    Assistant,

    #[db_rename = "STAFF"]
    Staff,
}

//
// Professor
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ProfessorPosition"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfessorPosition {
    #[db_rename = "PROFESSOR"]
    Professor,

    #[db_rename = "INVITED"]
    Invited,

    #[db_rename = "CONCURRENT"]
    Concurrent,

    #[db_rename = "VISITING"]
    Visiting,

    #[db_rename = "EMERITUS"]
    Emeritus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ProfessorStatus"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfessorStatus {
    #[db_rename = "ACTIVE"]
    Active,

    #[db_rename = "INACTIVE"]
    Inactive,
}

//
// Semester
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::SemesterType"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SemesterType {
    #[db_rename = "FIRST"]
    First,

    #[db_rename = "SUMMER"]
    Summer,

    #[db_rename = "SECOND"]
    Second,

    #[db_rename = "WINTER"]
    Winter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::SemesterStatus"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SemesterStatus {
    #[db_rename = "OPEN"]
    Open,

    #[db_rename = "CLOSED"]
    Closed,
}

//
// Master Course
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::CourseType"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CourseType {
    #[db_rename = "UNDERGRADUATE"]
    Undergraduate,

    #[db_rename = "GRADUATE"]
    Graduate,
}

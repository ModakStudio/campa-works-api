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

//
// Professor Quota
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::QuotaType"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuotaType {
    #[db_rename = "CREDIT"]
    Credit,

    #[db_rename = "HOUR"]
    Hour,
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
// Major
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MajorStatus"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MajorStatus {
    #[db_rename = "ACTIVE"]
    Active,

    #[db_rename = "INACTIVE"]
    Inactive,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::CourseStatus"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CourseStatus {
    #[db_rename = "ACTIVE"]
    Active,

    #[db_rename = "INACTIVE"]
    Inactive,
}

//
// Course
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::CourseCategory"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CourseCategory {
    #[db_rename = "MAJOR_REQUIRED"]
    MajorRequired,

    #[db_rename = "MAJOR_ELECTIVE"]
    MajorElective,

    #[db_rename = "GENERAL_REQUIRED"]
    GeneralRequired,

    #[db_rename = "GENERAL_ELECTIVE"]
    GeneralElective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Language"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    #[db_rename = "KOREAN"]
    Korean,

    #[db_rename = "ENGLISH"]
    English,
}

//
// Timetable
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::DayOfWeek"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    #[db_rename = "MON"]
    Mon,

    #[db_rename = "TUE"]
    Tue,

    #[db_rename = "WED"]
    Wed,

    #[db_rename = "THU"]
    Thu,

    #[db_rename = "FRI"]
    Fri,

    #[db_rename = "SAT"]
    Sat,

    #[db_rename = "SUN"]
    Sun,
}

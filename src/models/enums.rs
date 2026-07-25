use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

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

// We will use this in professor table
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
// #[ExistingTypePath = "crate::schema::sql_types::UserStatus"]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum UserStatus {
//     #[db_rename = "ACTIVE"]
//     Active,

//     #[db_rename = "INACTIVE"]
//     Inactive,
// }
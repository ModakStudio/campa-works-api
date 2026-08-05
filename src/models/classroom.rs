use diesel::prelude::*;

use crate::schema::classroom;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = classroom)]
pub struct Classroom {
    pub id: i64,

    pub building: String,
    pub room: String,

    pub capacity: i32,
    pub is_available: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = classroom)]
pub struct NewClassroom {
    pub building: String,
    pub room: String,

    pub capacity: i32,
    pub is_available: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = classroom)]
pub struct UpdateClassroom {
    pub building: Option<String>,
    pub room: Option<String>,

    pub capacity: Option<i32>,
    pub is_available: Option<bool>,
}

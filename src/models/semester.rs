use diesel::prelude::*;

use crate::{
    models::enums::{SemesterStatus, SemesterType},
    schema::semester,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = semester)]
pub struct Semester {
    pub id: i64,

    pub year: i32,
    pub semester_: SemesterType,

    pub semester_status: SemesterStatus,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = semester)]
pub struct NewSemester {
    pub year: i32,
    pub semester_: SemesterType,

    pub semester_status: SemesterStatus,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = semester)]
pub struct UpdateSemester {
    pub year: Option<i32>,
    pub semester_: Option<SemesterType>,

    pub semester_status: Option<SemesterStatus>,
}

use diesel::prelude::*;

use crate::schema::professor_credit;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = professor_credit)]
pub struct ProfessorCredit {
    pub id: i64,

    pub professor_id: i64,
    pub semester_id: i64,

    pub target_credit: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = professor_credit)]
pub struct NewProfessorCredit {
    pub professor_id: i64,
    pub semester_id: i64,

    pub target_credit: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = professor_credit)]
pub struct UpdateProfessorCredit {
    pub target_credit: Option<i32>,
}

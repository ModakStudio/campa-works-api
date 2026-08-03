use diesel::prelude::*;

use crate::{
    models::enums::{ProfessorPosition, ProfessorStatus},
    schema::professor,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = professor)]
pub struct Professor {
    pub id: i64,

    pub user_id: i64,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: ProfessorStatus,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = professor)]
pub struct NewProfessor {
    pub user_id: i64,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: ProfessorStatus,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = professor)]
pub struct UpdateProfessor {
    pub position: Option<ProfessorPosition>,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: Option<ProfessorStatus>,
}

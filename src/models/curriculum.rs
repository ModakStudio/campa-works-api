use diesel::prelude::*;

use crate::schema::curriculum;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = curriculum)]
pub struct Curriculum {
    pub id: i64,

    pub semester_id: i64,
    pub major_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = curriculum)]
pub struct NewCurriculum {
    pub semester_id: i64,
    pub major_id: i64,
}

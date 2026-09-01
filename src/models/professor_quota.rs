use diesel::prelude::*;

use crate::{models::enums::QuotaType, schema::professor_quota};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = professor_quota)]
pub struct ProfessorQuota {
    pub id: i64,

    pub professor_id: i64,
    pub semester_id: i64,

    pub quota_type: QuotaType,
    pub quota_value: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = professor_quota)]
pub struct NewProfessorQuota {
    pub professor_id: i64,
    pub semester_id: i64,

    pub quota_type: QuotaType,
    pub quota_value: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = professor_quota)]
pub struct UpdateProfessorQuota {
    pub quota_type: Option<QuotaType>,
    pub quota_value: Option<i32>,
}

use diesel::prelude::*;

use crate::{models::enums::MajorStatus, schema::major};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = major)]
pub struct Major {
    pub id: i64,

    pub major_name: String,
    pub major_code: String,

    pub major_status: MajorStatus,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = major)]
pub struct NewMajor {
    pub major_name: String,
    pub major_code: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = major)]
pub struct UpdateMajor {
    pub major_name: Option<String>,
    pub major_code: Option<String>,

    pub major_status: Option<MajorStatus>,
}

use diesel::prelude::*;

use crate::schema::facility;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = facility)]
pub struct Facility {
    pub id: i64,

    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = facility)]
pub struct NewFacility {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = facility)]
pub struct UpdateFacility {
    pub name: Option<String>,
    pub description: Option<String>,
}

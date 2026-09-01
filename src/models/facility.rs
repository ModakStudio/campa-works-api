use diesel::prelude::*;

use crate::schema::facility;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = facility)]
pub struct Facility {
    pub id: i64,

    pub facility_name: String,
    pub facility_description: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = facility)]
pub struct NewFacility {
    pub facility_name: String,
    pub facility_description: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = facility)]
pub struct UpdateFacility {
    pub facility_name: Option<String>,
    pub facility_description: Option<String>,
}

use diesel::prelude::*;

use crate::schema::classroom_facility;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = classroom_facility)]
pub struct ClassroomFacility {
    pub id: i64,

    pub classroom_id: i64,
    pub facility_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = classroom_facility)]
pub struct NewClassroomFacility {
    pub classroom_id: i64,
    pub facility_id: i64,
}
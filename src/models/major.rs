use diesel::prelude::*;

use crate::schema::major;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = major)]
pub struct Major {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = major)]
pub struct NewMajor {
    pub name: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = major)]
pub struct UpdateMajor {
    pub name: Option<String>,
}

use diesel::prelude::*;

use crate::schema::timetable;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = timetable)]
pub struct Timetable {
    pub id: i64,

    pub assignment_id: i64,
    pub classroom_id: i64,

    pub day_of_week: i32,

    pub start_period: i32,
    pub end_period: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = timetable)]
pub struct NewTimetable {
    pub assignment_id: i64,
    pub classroom_id: i64,

    pub day_of_week: i32,

    pub start_period: i32,
    pub end_period: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = timetable)]
pub struct UpdateTimetable {
    pub assignment_id: Option<i64>,
    pub classroom_id: Option<i64>,

    pub day_of_week: Option<i32>,

    pub start_period: Option<i32>,
    pub end_period: Option<i32>,
}

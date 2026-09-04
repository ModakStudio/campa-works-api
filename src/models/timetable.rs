use chrono::NaiveTime;
use diesel::prelude::*;

use crate::{models::enums::DayOfWeek, schema::timetable};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = timetable)]
pub struct Timetable {
    pub id: i64,

    pub assignment_id: i64,
    pub classroom_id: i64,

    pub day_of_week: DayOfWeek,

    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = timetable)]
pub struct NewTimetable {
    pub assignment_id: i64,
    pub classroom_id: i64,

    pub day_of_week: DayOfWeek,

    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = timetable)]
pub struct UpdateTimetable {
    pub assignment_id: Option<i64>,
    pub classroom_id: Option<i64>,

    pub day_of_week: Option<DayOfWeek>,

    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}

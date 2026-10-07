use chrono::NaiveTime;
use diesel::prelude::*;

use crate::{
    models::enums::DayOfWeek,
    schema::{basic_timetable_model, basic_timetable_slot, model_component},
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = basic_timetable_model)]
pub struct BasicTimetableModel {
    pub id: i64,
    pub total_min: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = basic_timetable_model)]
pub struct NewBasicTimetableModel {
    pub total_min: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = basic_timetable_model)]
pub struct UpdateBasicTimetableModel {
    pub total_min: Option<i32>,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Clone)]
#[diesel(table_name = basic_timetable_slot)]
pub struct BasicTimetableSlot {
    pub id: i64,
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = basic_timetable_slot)]
pub struct NewBasicTimetableSlot {
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = model_component)]
pub struct ModelComponent {
    pub id: i64,
    pub basic_timetable_model_id: i64,
    pub basic_timetable_slot_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = model_component)]
pub struct NewModelComponent {
    pub basic_timetable_model_id: i64,
    pub basic_timetable_slot_id: i64,
}

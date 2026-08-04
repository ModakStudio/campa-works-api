use diesel::prelude::*;

use crate::{
    models::enums::{CourseCategory, Language},
    schema::course,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course)]
pub struct Course {
    pub id: i64,

    pub master_course_id: i64,
    pub semester_id: i64,
    pub major_id: i64,

    pub description: Option<String>,

    pub course_category: CourseCategory,

    pub language: Language,

    pub section_number: i32,
    pub grade: i32,
    pub capacity: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course)]
pub struct NewCourse {
    pub master_course_id: i64,
    pub semester_id: i64,
    pub major_id: i64,

    pub description: Option<String>,

    pub course_category: CourseCategory,

    pub language: Language,

    pub section_number: i32,
    pub grade: i32,
    pub capacity: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = course)]
pub struct UpdateCourse {
    pub description: Option<String>,

    pub course_category: Option<CourseCategory>,

    pub language: Option<Language>,

    pub section_number: Option<i32>,
    pub grade: Option<i32>,
    pub capacity: Option<i32>,
}

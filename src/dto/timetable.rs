use serde::{Deserialize, Serialize};

use crate::{
    dto::{classroom::ClassroomResponse, course_assignment::CourseAssignmentResponse},
    models::{
        classroom::Classroom, course::Course, course_assignment::CourseAssignment, major::Major,
        master_course::MasterCourse, professor::Professor, semester::Semester,
        timetable::Timetable, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateTimetableRequest {
    pub assignment_id: i64,
    pub classroom_id: i64,

    pub day_of_week: i32,

    pub start_period: i32,
    pub end_period: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTimetableRequest {
    pub assignment_id: Option<i64>,
    pub classroom_id: Option<i64>,

    pub day_of_week: Option<i32>,

    pub start_period: Option<i32>,
    pub end_period: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TimetableResponse {
    pub id: i64,

    pub assignment: CourseAssignmentResponse,
    pub classroom: ClassroomResponse,

    pub day_of_week: i32,

    pub start_period: i32,
    pub end_period: i32,
}

impl
    From<(
        Timetable,
        CourseAssignment,
        Course,
        MasterCourse,
        Semester,
        Major,
        Professor,
        User,
        Classroom,
    )> for TimetableResponse
{
    fn from(
        (timetable, assignment, course, master_course, semester, major, professor, user, classroom): (
            Timetable, CourseAssignment, Course, MasterCourse, Semester, Major, Professor, User, Classroom
        ),
    ) -> Self {
        Self {
            id: timetable.id,

            assignment: CourseAssignmentResponse::from((
                assignment,
                course,
                master_course,
                semester,
                major,
                professor,
                user,
            )),
            classroom: ClassroomResponse::from(classroom),

            day_of_week: timetable.day_of_week,

            start_period: timetable.start_period,
            end_period: timetable.end_period,
        }
    }
}

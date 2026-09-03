use serde::{Deserialize, Serialize};

use crate::{
    dto::{classroom::ClassroomResponse, course_assignment::CourseAssignmentResponse},
    models::{
        classroom::Classroom, course::Course, course_assignment::CourseAssignment,
        enums::DayOfWeek, master_course::MasterCourse, professor::Professor, semester::Semester,
        timetable::Timetable, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateTimetableRequest {
    pub assignment_id: i64,
    pub classroom_id: i64,

    pub day_of_week: DayOfWeek,

    pub start_period: i32,
    pub end_period: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTimetableRequest {
    pub assignment_id: Option<i64>,
    pub classroom_id: Option<i64>,

    pub day_of_week: Option<DayOfWeek>,

    pub start_period: Option<i32>,
    pub end_period: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TimetableResponse {
    pub id: i64,

    pub assignment: CourseAssignmentResponse,
    pub classroom: ClassroomResponse,

    pub day_of_week: DayOfWeek,

    pub start_period: i32,
    pub end_period: i32,
}

impl
    From<(
        Timetable,
        CourseAssignment,
        Course,
        MasterCourse,
        Professor,
        User,
        Semester,
        Classroom,
    )> for TimetableResponse
{
    fn from(
        (timetable, assignment, course, master_course, professor, user, semester, classroom): (
            Timetable,
            CourseAssignment,
            Course,
            MasterCourse,
            Professor,
            User,
            Semester,
            Classroom,
        ),
    ) -> Self {
        Self {
            id: timetable.id,

            assignment: CourseAssignmentResponse::from((
                assignment,
                course,
                master_course,
                professor,
                user,
                semester,
            )),
            classroom: ClassroomResponse::from(classroom),

            day_of_week: timetable.day_of_week,

            start_period: timetable.start_period,
            end_period: timetable.end_period,
        }
    }
}

impl From<&str> for DayOfWeek {
    fn from(s: &str) -> Self {
        match s {
            "MON" => DayOfWeek::Mon,
            "TUES" => DayOfWeek::Tue,
            "WED" => DayOfWeek::Wed,
            "THURS" => DayOfWeek::Thu,
            "FRI" => DayOfWeek::Fri,
            "SAT" => DayOfWeek::Sat,
            "SUN" => DayOfWeek::Sun,
            _ => panic!("Invalid day of week: {}", s),
        }
    }
}

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DayOfWeek::Mon => "MON",
            DayOfWeek::Tue => "TUE",
            DayOfWeek::Wed => "WED",
            DayOfWeek::Thu => "THU",
            DayOfWeek::Fri => "FRI",
            DayOfWeek::Sat => "SAT",
            DayOfWeek::Sun => "SUN",
        };
        write!(f, "{}", s)
    }
}

use std::collections::{HashMap, HashSet};

use chrono::NaiveTime;
use diesel::prelude::*;

use crate::{
    dto::timetable::{CreateTimetableRequest, TimetableResponse, UpdateTimetableRequest},
    error::app_error::AppError,
    models::{
        basic_timetable::BasicTimetableSlot,
        enums::DayOfWeek,
        timetable::{NewTimetable, UpdateTimetable},
    },
    repository::{
        basic_timetable_repository::BasicTimetableRepository,
        classroom_facility_repository::ClassroomFacilityRepository,
        classroom_repository::ClassroomRepository,
        course_assignment_repository::CourseAssignmentRepository,
        course_facility_repository::CourseFacilityRepository,
        timetable_repository::TimetableRepository,
    },
    service::{course_assignment_service::CourseAssignmentService, course_service::CourseService},
};

#[derive(Clone, Copy)]
struct ScheduleSlot {
    day_of_week: DayOfWeek,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

struct ScheduleCandidate {
    classroom_id: i64,
    slots: Vec<ScheduleSlot>,
}

struct CourseScheduleRequest {
    assignment_id: i64,
    professor_id: i64,
    candidates: Vec<ScheduleCandidate>,
}

#[derive(Clone, Copy)]
struct ScheduledSlot {
    professor_id: i64,
    classroom_id: i64,
    day_of_week: DayOfWeek,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

impl From<&BasicTimetableSlot> for ScheduleSlot {
    fn from(slot: &BasicTimetableSlot) -> Self {
        Self {
            day_of_week: slot.day_of_week,
            start_time: slot.start_time,
            end_time: slot.end_time,
        }
    }
}

fn periods_overlap(
    first_start: NaiveTime,
    first_end: NaiveTime,
    second_start: NaiveTime,
    second_end: NaiveTime,
) -> bool {
    first_start < second_end && first_end > second_start
}

fn assign_schedule_candidates(
    requests: &[CourseScheduleRequest],
    request_idx: usize,
    occupied_slots: &mut Vec<ScheduledSlot>,
    planned_timetables: &mut Vec<NewTimetable>,
) -> bool {
    if request_idx == requests.len() {
        return true;
    }

    let request = &requests[request_idx];
    for candidate in &request.candidates {
        let has_conflict = candidate.slots.iter().any(|candidate_slot| {
            occupied_slots.iter().any(|slot| {
                slot.day_of_week == candidate_slot.day_of_week
                    && periods_overlap(
                        slot.start_time,
                        slot.end_time,
                        candidate_slot.start_time,
                        candidate_slot.end_time,
                    )
                    && (slot.professor_id == request.professor_id
                        || slot.classroom_id == candidate.classroom_id)
            })
        });
        if has_conflict {
            continue;
        }

        let occupied_slots_len = occupied_slots.len();
        let planned_timetables_len = planned_timetables.len();
        for slot in &candidate.slots {
            occupied_slots.push(ScheduledSlot {
                professor_id: request.professor_id,
                classroom_id: candidate.classroom_id,
                day_of_week: slot.day_of_week,
                start_time: slot.start_time,
                end_time: slot.end_time,
            });
            planned_timetables.push(NewTimetable {
                assignment_id: request.assignment_id,
                classroom_id: candidate.classroom_id,
                day_of_week: slot.day_of_week,
                start_time: slot.start_time,
                end_time: slot.end_time,
            });
        }

        if assign_schedule_candidates(
            requests,
            request_idx + 1,
            occupied_slots,
            planned_timetables,
        ) {
            return true;
        }

        occupied_slots.truncate(occupied_slots_len);
        planned_timetables.truncate(planned_timetables_len);
    }

    false
}

pub struct TimetableService;

impl TimetableService {
    pub fn create(
        conn: &mut PgConnection,
        request: CreateTimetableRequest,
    ) -> Result<TimetableResponse, AppError> {
        CourseAssignmentRepository::find_by_id(conn, request.assignment_id)
            .map_err(|_| AppError::CourseAssignmentNotFound)?;

        ClassroomRepository::find_by_id(conn, request.classroom_id)
            .map_err(|_| AppError::ClassroomNotFound)?;

        if TimetableRepository::find_overlapping_timetables(
            conn,
            request.classroom_id,
            request.day_of_week,
            request.start_time,
            request.end_time,
        )
        .map_err(|_| AppError::DatabaseError)?
        .len()
            > 0
        {
            return Err(AppError::TimetableOverlap);
        }

        let new_timetable = NewTimetable {
            assignment_id: request.assignment_id,
            classroom_id: request.classroom_id,
            day_of_week: request.day_of_week,
            start_time: request.start_time,
            end_time: request.end_time,
        };

        let timetable_id = TimetableRepository::create(conn, &new_timetable)
            .map_err(|_| AppError::DatabaseError)?
            .id;
        let timetable = TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(timetable.into())
    }

    pub fn create_all_in_new_semester(
        conn: &mut PgConnection,
        new_semester_id: i64,
    ) -> Result<(), AppError> {
        // Create all courses in the new semester
        CourseService::create_all_in_new_semester(conn, new_semester_id)?;

        // Assign professors to courses in the new semester
        CourseAssignmentService::create_auto_in_new_semester(conn, new_semester_id)?;

        Self::create_auto_timetables(conn, new_semester_id)?;

        Ok(())
    }

    fn create_auto_timetables(conn: &mut PgConnection, semester_id: i64) -> Result<(), AppError> {
        let assignments = CourseAssignmentRepository::find_all(
            conn,
            &HashMap::from([("semester_id".to_string(), semester_id.to_string())]),
        )
        .map_err(|_| AppError::DatabaseError)?;
        let classrooms = ClassroomRepository::find_all(
            conn,
            &HashMap::from([("is_available".to_string(), "true".to_string())]),
        )
        .map_err(|_| AppError::DatabaseError)?;

        let mut models_by_total_min = HashMap::<i32, Vec<Vec<ScheduleSlot>>>::new();
        for (model, slots) in
            BasicTimetableRepository::find_all(conn).map_err(|_| AppError::DatabaseError)?
        {
            let model_slots: Vec<_> = slots.iter().map(ScheduleSlot::from).collect();
            let total_min = model_slots
                .iter()
                .map(|slot| (slot.end_time - slot.start_time).num_minutes())
                .sum::<i64>();
            let invalid_slot = model_slots
                .iter()
                .any(|slot| slot.start_time >= slot.end_time);
            let overlapping_slots = model_slots.iter().enumerate().any(|(index, slot)| {
                model_slots[index + 1..].iter().any(|other| {
                    slot.day_of_week == other.day_of_week
                        && periods_overlap(
                            slot.start_time,
                            slot.end_time,
                            other.start_time,
                            other.end_time,
                        )
                })
            });

            if !model_slots.is_empty()
                && !invalid_slot
                && !overlapping_slots
                && total_min == i64::from(model.total_min)
            {
                models_by_total_min
                    .entry(model.total_min)
                    .or_default()
                    .push(model_slots);
            }
        }

        let mut classroom_facilities = HashMap::<i64, HashSet<i64>>::new();
        for classroom in &classrooms {
            let facility_ids = ClassroomFacilityRepository::find_all(
                conn,
                &HashMap::from([("classroom_id".to_string(), classroom.id.to_string())]),
            )
            .map_err(|_| AppError::DatabaseError)?
            .into_iter()
            .map(|(classroom_facility, _, _)| classroom_facility.facility_id)
            .collect();
            classroom_facilities.insert(classroom.id, facility_ids);
        }

        let existing_timetables = TimetableRepository::find_all(conn, &HashMap::new())
            .map_err(|_| AppError::DatabaseError)?;
        let mut existing_assignment_ids = HashSet::new();
        let mut occupied_slots = Vec::new();
        for (timetable, _, _, _, _, _, semester, _, professor, _, _) in existing_timetables {
            if semester.id != semester_id {
                continue;
            }

            existing_assignment_ids.insert(timetable.assignment_id);
            occupied_slots.push(ScheduledSlot {
                professor_id: professor.id,
                classroom_id: timetable.classroom_id,
                day_of_week: timetable.day_of_week,
                start_time: timetable.start_time,
                end_time: timetable.end_time,
            });
        }

        let mut course_facilities = HashMap::<i64, HashSet<i64>>::new();
        let mut requests = Vec::new();
        for (assignment, course, _, master_course, _, _, _, professor, _) in assignments {
            if existing_assignment_ids.contains(&assignment.id) {
                continue;
            }

            let total_min = course
                .lecture
                .checked_add(course.practice)
                .and_then(|hours| hours.checked_mul(60))
                .filter(|total_min| *total_min > 0)
                .ok_or(AppError::TimetableSchedulingFailed)?;
            let matching_models = models_by_total_min
                .get(&total_min)
                .ok_or(AppError::TimetableSchedulingFailed)?;

            let required_facilities =
                if let Some(facilities) = course_facilities.get(&master_course.id) {
                    facilities.clone()
                } else {
                    let facilities = CourseFacilityRepository::find_all(
                        conn,
                        &HashMap::from([(
                            "master_course_id".to_string(),
                            master_course.id.to_string(),
                        )]),
                    )
                    .map_err(|_| AppError::DatabaseError)?
                    .into_iter()
                    .map(|(course_facility, _, _)| course_facility.facility_id)
                    .collect::<HashSet<_>>();
                    course_facilities.insert(master_course.id, facilities.clone());
                    facilities
                };

            let mut compatible_classrooms: Vec<_> = classrooms
                .iter()
                .filter(|classroom| {
                    classroom.capacity >= course.capacity
                        && required_facilities.is_subset(
                            classroom_facilities
                                .get(&classroom.id)
                                .expect("facilities loaded for each available classroom"),
                        )
                })
                .collect();
            compatible_classrooms.sort_by_key(|classroom| classroom.capacity);

            let candidates: Vec<_> = matching_models
                .iter()
                .flat_map(|slots| {
                    compatible_classrooms
                        .iter()
                        .map(|classroom| ScheduleCandidate {
                            classroom_id: classroom.id,
                            slots: slots.clone(),
                        })
                })
                .collect();
            if candidates.is_empty() {
                return Err(AppError::TimetableSchedulingFailed);
            }

            requests.push(CourseScheduleRequest {
                assignment_id: assignment.id,
                professor_id: professor.id,
                candidates,
            });
        }

        requests.sort_by_key(|request| request.candidates.len());
        let mut planned_timetables = Vec::new();
        if !assign_schedule_candidates(&requests, 0, &mut occupied_slots, &mut planned_timetables) {
            return Err(AppError::TimetableSchedulingFailed);
        }

        for timetable in planned_timetables {
            TimetableRepository::create(conn, &timetable).map_err(|_| AppError::DatabaseError)?;
        }

        Ok(())
    }

    pub fn get_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> Result<Vec<TimetableResponse>, AppError> {
        let timetables =
            TimetableRepository::find_all(conn, params).map_err(|_| AppError::DatabaseError)?;

        Ok(timetables.into_iter().map(Into::into).collect())
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i64) -> Result<TimetableResponse, AppError> {
        let timetable =
            TimetableRepository::find_by_id(conn, id).map_err(|_| AppError::TimetableNotFound)?;

        Ok(timetable.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        timetable_id: i64,
        request: UpdateTimetableRequest,
    ) -> Result<TimetableResponse, AppError> {
        TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::TimetableNotFound)?;

        if let Some(assignment_id) = request.assignment_id {
            CourseAssignmentRepository::find_by_id(conn, assignment_id)
                .map_err(|_| AppError::CourseAssignmentNotFound)?;
        }

        if let Some(classroom_id) = request.classroom_id {
            ClassroomRepository::find_by_id(conn, classroom_id)
                .map_err(|_| AppError::ClassroomNotFound)?;
        }

        let updated_timetable = UpdateTimetable {
            assignment_id: request.assignment_id,
            classroom_id: request.classroom_id,

            day_of_week: request.day_of_week,

            start_time: request.start_time,
            end_time: request.end_time,
        };

        TimetableRepository::update(conn, timetable_id, &updated_timetable)
            .map_err(|_| AppError::DatabaseError)?;

        let updated_timetable = TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::DatabaseError)?;

        Ok(updated_timetable.into())
    }

    pub fn delete(conn: &mut PgConnection, timetable_id: i64) -> Result<(), AppError> {
        TimetableRepository::find_by_id(conn, timetable_id)
            .map_err(|_| AppError::TimetableNotFound)?;

        TimetableRepository::delete(conn, timetable_id).map_err(|_| AppError::DatabaseError)?;

        Ok(())
    }
}

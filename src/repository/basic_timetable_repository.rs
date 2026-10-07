use diesel::prelude::*;
use std::collections::HashSet;

use crate::{
    models::basic_timetable::{
        BasicTimetableModel, BasicTimetableSlot, NewBasicTimetableModel, NewBasicTimetableSlot,
        NewModelComponent, UpdateBasicTimetableModel,
    },
    schema::{basic_timetable_model, basic_timetable_slot, model_component},
};

pub struct BasicTimetableRepository;

impl BasicTimetableRepository {
    pub fn find_all(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(BasicTimetableModel, Vec<BasicTimetableSlot>)>> {
        let models = basic_timetable_model::table
            .select(BasicTimetableModel::as_select())
            .order_by(basic_timetable_model::id)
            .load(conn)?;

        models
            .into_iter()
            .map(|model| Self::find_slots_by_model_id(conn, model.id).map(|slots| (model, slots)))
            .collect()
    }

    pub fn create(
        conn: &mut PgConnection,
        new_model: &NewBasicTimetableModel,
        slots: &[NewBasicTimetableSlot],
    ) -> QueryResult<BasicTimetableModel> {
        conn.transaction(|conn| {
            let model = diesel::insert_into(basic_timetable_model::table)
                .values(new_model)
                .returning(BasicTimetableModel::as_returning())
                .get_result(conn)?;

            let slot_ids = Self::find_or_create_slot_ids(conn, slots)?;
            Self::replace_components(conn, model.id, &slot_ids)?;

            Ok(model)
        })
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        model_id: i64,
    ) -> QueryResult<(BasicTimetableModel, Vec<BasicTimetableSlot>)> {
        let model = basic_timetable_model::table
            .find(model_id)
            .select(BasicTimetableModel::as_select())
            .first(conn)?;
        let slots = Self::find_slots_by_model_id(conn, model_id)?;

        Ok((model, slots))
    }

    fn find_slots_by_model_id(
        conn: &mut PgConnection,
        model_id: i64,
    ) -> QueryResult<Vec<BasicTimetableSlot>> {
        basic_timetable_slot::table
            .inner_join(
                model_component::table
                    .on(model_component::basic_timetable_slot_id.eq(basic_timetable_slot::id)),
            )
            .filter(model_component::basic_timetable_model_id.eq(model_id))
            .select(BasicTimetableSlot::as_select())
            .order_by(basic_timetable_slot::id)
            .load(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        model_id: i64,
        changes: &UpdateBasicTimetableModel,
        slots: Option<&[NewBasicTimetableSlot]>,
    ) -> QueryResult<(BasicTimetableModel, Vec<BasicTimetableSlot>)> {
        conn.transaction(|conn| {
            diesel::update(basic_timetable_model::table.find(model_id))
                .set(changes)
                .execute(conn)?;

            if let Some(slots) = slots {
                let slot_ids = Self::find_or_create_slot_ids(conn, slots)?;
                Self::replace_components(conn, model_id, &slot_ids)?;
            }

            Self::find_by_id(conn, model_id)
        })
    }

    pub fn delete(conn: &mut PgConnection, model_id: i64) -> QueryResult<usize> {
        diesel::delete(basic_timetable_model::table.find(model_id)).execute(conn)
    }

    fn find_or_create_slot_ids(
        conn: &mut PgConnection,
        slots: &[NewBasicTimetableSlot],
    ) -> QueryResult<Vec<i64>> {
        slots
            .iter()
            .map(|slot| {
                diesel::insert_into(basic_timetable_slot::table)
                    .values(slot)
                    .on_conflict((
                        basic_timetable_slot::day_of_week,
                        basic_timetable_slot::start_time,
                        basic_timetable_slot::end_time,
                    ))
                    .do_update()
                    .set(basic_timetable_slot::id.eq(basic_timetable_slot::id))
                    .returning(basic_timetable_slot::id)
                    .get_result(conn)
            })
            .collect()
    }

    fn replace_components(
        conn: &mut PgConnection,
        model_id: i64,
        slot_ids: &[i64],
    ) -> QueryResult<()> {
        diesel::delete(
            model_component::table.filter(model_component::basic_timetable_model_id.eq(model_id)),
        )
        .execute(conn)?;

        let mut unique_slot_ids = HashSet::new();
        let components = slot_ids
            .iter()
            .filter(|slot_id| unique_slot_ids.insert(**slot_id))
            .map(|slot_id| NewModelComponent {
                basic_timetable_model_id: model_id,
                basic_timetable_slot_id: *slot_id,
            })
            .collect::<Vec<_>>();

        if !components.is_empty() {
            diesel::insert_into(model_component::table)
                .values(&components)
                .execute(conn)?;
        }

        Ok(())
    }
}

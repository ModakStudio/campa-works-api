use diesel::prelude::*;
use std::collections::HashMap;

use crate::{
    models::{
        enums::*,
        professor::Professor,
        professor_quota::{NewProfessorQuota, ProfessorQuota, UpdateProfessorQuota},
        semester::Semester,
        user::User,
    },
    schema::{professor, professor_quota, semester, users},
};

#[macro_export]
macro_rules! apply_professor_quota_query_filters {
    ($query:expr, $params:expr) => {{
        let mut query = $query;

        if let Some(professor_quota_id) = $params
            .get("id")
            .and_then(|value| value.parse::<i64>().ok())
        {
            query = query.filter(professor_quota::id.eq(professor_quota_id));
        }

        query = crate::apply_professor_query_filters!(query, $params);

        query = crate::apply_semester_query_filters!(query, $params);

        if let Some(quota_type) = $params
            .get("quota_type")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            query = query.filter(professor_quota::quota_type.eq(QuotaType::from(quota_type)));
        }
        if let Some(quota_value) = $params
            .get("quota_value")
            .and_then(|value| value.parse::<i32>().ok())
        {
            query = query.filter(professor_quota::quota_value.eq(quota_value));
        }

        query
    }};
}

pub struct ProfessorQuotaRepository;

impl ProfessorQuotaRepository {
    pub fn create(
        conn: &mut PgConnection,
        new_professor_quota: &NewProfessorQuota,
    ) -> QueryResult<ProfessorQuota> {
        diesel::insert_into(professor_quota::table)
            .values(new_professor_quota)
            .returning(ProfessorQuota::as_returning())
            .get_result(conn)
    }

    pub fn find_all(
        conn: &mut PgConnection,
        params: &HashMap<String, String>,
    ) -> QueryResult<Vec<(ProfessorQuota, Professor, User, Semester)>> {
        let mut query = professor_quota::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .into_boxed();

        query = apply_professor_quota_query_filters!(query, params);

        query
            .select((
                ProfessorQuota::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .load(conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        professor_quota_id: i64,
    ) -> QueryResult<(ProfessorQuota, Professor, User, Semester)> {
        professor_quota::table
            .inner_join(professor::table.inner_join(users::table))
            .inner_join(semester::table)
            .filter(professor_quota::id.eq(professor_quota_id))
            .select((
                ProfessorQuota::as_select(),
                Professor::as_select(),
                User::as_select(),
                Semester::as_select(),
            ))
            .first(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        professor_quota_id: i64,
        update_professor_quota: &UpdateProfessorQuota,
    ) -> QueryResult<ProfessorQuota> {
        diesel::update(professor_quota::table.filter(professor_quota::id.eq(professor_quota_id)))
            .set(update_professor_quota)
            .returning(ProfessorQuota::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, professor_quota_id: i64) -> QueryResult<usize> {
        diesel::delete(professor_quota::table.filter(professor_quota::id.eq(professor_quota_id)))
            .execute(conn)
    }
}

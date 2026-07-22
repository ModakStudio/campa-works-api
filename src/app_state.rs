use crate::db::pool::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

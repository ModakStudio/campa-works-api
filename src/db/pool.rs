use deadpool_diesel::{
    Runtime,
    postgres::{Manager, Pool},
};

use crate::config::database::DatabaseConfig;

pub type DbPool = Pool;

pub fn create_pool() -> DbPool {
    let config = DatabaseConfig::from_env();

    let manager = Manager::new(config.database_url, Runtime::Tokio1);

    Pool::builder(manager)
        .max_size(16)
        .build()
        .expect("Failed to create DB Pool")
}

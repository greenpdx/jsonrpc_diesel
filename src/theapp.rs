use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use slog::Logger;
use dotenv::dotenv;
use std::env;
use std::sync::atomic::{self, AtomicUsize};
use jsonrpc_core::*;
use jsonrpc_core::futures::Future;
use meta::Meta;
use std::time::Instant;

pub type DieselPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct DieselMidWare {
    pub pool: DieselPool
}
impl DieselMidWare {
	pub fn new (logger: &Logger) -> DieselMidWare{
		let logger = logger.new(o!("module" => "DieselMidWare"));
        dotenv().ok();

		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

		let config = r2d2::Config::default();
		let manager = ConnectionManager::<SqliteConnection>::new(database_url);
		let pool = r2d2::Pool::new(config, manager).expect("Failed to create diesel pool.");

		info!(logger, "Diesel pool created");

		DieselMidWare {pool: pool}
	}
//    pub fn get(&self) -> Option<DieselConnection> {
//        let ref pool = self.pool;
//        Some(pool.unwrap().get().unwrap())
//    }
}

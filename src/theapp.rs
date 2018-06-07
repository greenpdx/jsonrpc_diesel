
use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
//use r2d2_sqlite::SqliteConnectionManager;
use slog::Logger;
use dotenv::dotenv;
use std::env;
use std::sync::atomic::{self, AtomicUsize};
use jsonrpc_core::*;
use chrono::prelude::*;
use jsonrpc_core::futures::Future;
use meta::Meta;
use std::time::Instant;
use keycap::{capkey, Sales};
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub type DieselPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct DieselMidWare {
    pub pool: DieselPool,
    //data: Arc<RwLock<Sales>>
}
impl DieselMidWare {
	pub fn new (logger: &Logger) -> DieselMidWare {
		let logger = logger.new(o!("module" => "DieselMidWare"));
        dotenv().ok();

		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //let config = r2d2::Config::default();
        let manager = ConnectionManager::new(database_url);
        //println!("{:?}", manager );
        //let pool = r2d2::Pool::new(config, manager).expect("Failed to create diesel pool.");
        let pool = r2d2::Pool::builder().build(manager).expect("No Pool");
//        let sales = Sales::default();
//        let mut tsales = Arc::new(RwLock::new(sales));

        let (tx, _rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
        let mut sale = pool.clone();
        let _chld = thread::spawn(move || capkey(&sale,tx));

			info!(logger, "Diesel pool created");

		DieselMidWare {pool: pool}
	}
    pub fn sale (&mut self, key: u16) {
        let ts = Local::now().timestamp();
        println!("Sale {:?}", key)
        //let mut f = &self.fil;
        //write!(f, "{:?} {:?}", key, ts);
    }

//    pub fn get(&self) -> Option<DieselConnection> {
//        let ref pool = self.pool;
//        Some(pool.unwrap().get().unwrap())
//    }
}

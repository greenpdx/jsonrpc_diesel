use diesel::pg::PgConnection;
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

pub type DieselPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

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
		let manager = ConnectionManager::<PgConnection>::new(database_url);
		let pool = r2d2::Pool::new(config, manager).expect("Failed to create diesel pool.");

		info!(logger, "Diesel pool created");

		DieselMidWare {pool: pool}
	}
//    pub fn get(&self) -> Option<DieselConnection> {
//        let ref pool = self.pool;
//        Some(pool.unwrap().get().unwrap())
//    }
}

#[derive(Default)]
pub struct MyMiddleware(AtomicUsize);
impl Middleware<Meta> for MyMiddleware {
	type Future = FutureResponse;

	fn on_request<F, X>(&self, request: Request, meta: Meta, next: F) -> FutureResponse where
		F: FnOnce(Request, Meta) -> X + Send,
		X: Future<Item=Option<Response>, Error=()> + Send + 'static,
	{
        let m = meta.clone();
        let logger = m.logger.unwrap().clone();
		let start = Instant::now();
		let request_number = self.0.fetch_add(1, atomic::Ordering::SeqCst);
		//println!("Processing request {}: {:?}, {:?}", request_number, request, meta);

		Box::new(next(request, meta).map(move |res| {

            info!(logger, "{} Processing took: {:?}", request_number, start.elapsed());
			println!("Processing took: {:?}", start.elapsed());
			res
		}))
	}
}

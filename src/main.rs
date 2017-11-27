extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
// #[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
// #[macro_use] extern crate derive_builder;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate slog;
extern crate slog_term;
extern crate slog_json;

pub mod schema;
pub mod models;
mod utils;

use std::thread;
use jsonrpc_core::*;
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi, MetaExtractor, hyper};
use self::hyper::{Method, Uri, HttpVersion, Headers, Body, header};
use jsonrpc_core::futures::Future;
use std::sync::atomic::{self, AtomicUsize};
use std::time::Instant;
use std::net::{SocketAddr, IpAddr};
use std::path::Path;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::fmt;
use std::fmt::Formatter;
use diesel::pg::types::sql_types::*;
use diesel::types::Timestamp;
//use serde_json::builder;
use r2d2_diesel::ConnectionManager;
use slog::Logger;
use utils::logger_factory;

pub type DieselPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DieselMidWare {
    pool: DieselPool
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
/*
pub trait GetComm {
    fn get_conn(&self) ->&DieselConnection;
}

impl GetComm for DieselMidWare {
    fn get_conn(&self) -> &DieselConnection {
        let p = &self.pool;
        &p.unwrap().get().unwrap()
    }
}
*/
#[derive(Clone,Default)]
struct Meta {
    remote: Option<SocketAddr>,
//    bob: (Method, Uri, HttpVersion, Headers, Body),
    methd: Method,
    uri: Uri,
    hdrs: Headers,
    dbpool: Option<DieselMidWare>,
    logger: Option<Logger>,
//    path: String,
}
impl Metadata for Meta {}

impl fmt::Debug for Meta {
    fn fmt(&self,f: &mut Formatter) -> fmt::Result {
        let pgstr = "PG".to_string();
        write!(f, "{:?} {:?} {:?} {:?} {:?}", self.remote, self.methd, self.uri, self.hdrs, pgstr)
    }
}
#[derive(Default)]
struct MyMiddleware(AtomicUsize);
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

            info!(logger, "Processing took: {:?}", start.elapsed());
			println!("Processing took: {:?}", start.elapsed());
			res
		}))
	}
}

fn create_tst<'a>(conn: &PgConnection, cmd: String, rpcid: i32)  {
    use self::schema::tst1;
    let new_tst = &models::NewTst {
        methd: cmd,
        rpcid: rpcid,
    };
//    let tst: models::Tst1 = diesel::insert_into(tst1::table)
//        .values(cmd)
//        .get_results(conn)
//        .expect("Error");
    let tst = diesel::insert(new_tst)
        .into(tst1::table)
        .execute(conn)
        .expect("Error");

//    tst
}

fn methd_ins(params: Params, meta: Meta) -> Result<Value> {
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let js: [i32;2] = params.parse().unwrap();
//    println!("INS {:?} {:?}", js, meta);
    let rslt = create_tst(&*conn, "say_ins".to_string(), 1);
    Ok(Value::String(format!("{:?}", js)))
}


fn methd_qry(params: Params, meta: Meta) -> Result<Value> {
    use self::schema::tst1::dsl::*;
    use models::Tst1;
    use serde::Serialize;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let rslt = tst1.filter(id.ne(0))
        .load::<Tst1>(&*conn)
        .expect("Error");
    let r = json!(&rslt);
//    println!("{:?}", r);
    for itm in rslt {
        let s = serde_json::to_string(&itm).unwrap();
        println!("{} {} {}", itm.id, s, itm);
    }
    Ok(Value::String(format!("{}", &r)))
}

fn methd_bye(params: Params, meta: Meta) -> Result<Value> {
    let js: [i32;2] = params.parse().unwrap();
    Ok(Value::String(format!("{:?}", js)))
}
fn methd_more(params: Params, meta: Meta) -> Result<Value> {
    let js: [i32;2] = params.parse().unwrap();
    Ok(Value::String(format!("{:?} {:?}", js, meta)))
}

fn methd_hello(_parm: Params, meta: Meta) -> Result<Value> {
    let remote = meta.remote.unwrap(); // .unwrap_or_else(String::new);
    println!("Hello {:?} {:?}", _parm, remote );
    let ipaddr = remote.ip();
    let port = remote.port();
    let ans = format!("{}", remote);
    Ok(Value::String(ans))
}


fn main() {
    let logger = logger_factory();
    let thepool = DieselMidWare::new(&logger);

    let mut io = MetaIoHandler::with_middleware(MyMiddleware::default());

    io.add_method_with_meta("say_hello", &methd_hello);
    io.add_method_with_meta("say_bye", &methd_bye);
    io.add_method_with_meta("say_ins", &methd_ins);
    io.add_method_with_meta("say_qry", &methd_qry);
    io.add_method_with_meta("say_more", &methd_more);

    let _server = ServerBuilder::new(io)
//        .threads(3)
        .rest_api(RestApi::Unsecure)
        .meta_extractor( move |req: &hyper::Request| {
            let methd =req.method().clone();
            let uri = req.uri().clone();
            let hdrs = req.headers().clone();
            let remote = req.remote_addr().clone();
            let dbpool = thepool.clone();
            let logger = logger.clone();

            Meta { methd, uri, hdrs, remote, dbpool: Some(dbpool), logger: Some(logger) }
        })
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
        .start_http(&"0.0.0.0:3030".parse().unwrap())
        .expect("unable to start");

    _server.wait();
}

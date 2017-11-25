extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;

pub mod schema;
pub mod models;

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


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[derive(Clone)]
struct Meta {
    remote: Option<SocketAddr>,
//    bob: (Method, Uri, HttpVersion, Headers, Body),
    methd: Method,
    uri: Uri,
    hdrs: Headers,
//    pgconn: Option<PgConnection>,
//    path: String,
}
impl Metadata for Meta {}
impl Default for Meta {
    fn default() -> Meta {
        Meta {
            remote: Option::default(),
            methd: Method::default(),
            uri: Uri::default(),
            hdrs: Headers::default(),
//            pgconn: None,
        }
    }
}
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
		let start = Instant::now();
		let request_number = self.0.fetch_add(1, atomic::Ordering::SeqCst);
		println!("Processing request {}: {:?}, {:?}", request_number, request, meta);

		Box::new(next(request, meta).map(move |res| {
			println!("Processing took: {:?}", start.elapsed());
			res
		}))
	}
}

struct NewTst1 {
    js: Value,
}


fn create_tst<'a>(conn: &PgConnection, justj: Value) -> models::Tst1 {
    use self::schema::tst1;
    let new_tst = &NewTst1 {
        js: justj,
    };
    let tst: models::Tst1 = diesel::insert(new_tst).into(tst1::table)
        .get_result(conn)
        .expect("Error");
    tst
}

fn methd_pg(params: Params, meta: Meta) -> Result<Value> {

    let conn = establish_connection();
    let js: [i32;2] = params.parse().unwrap();
    let justj = serde_json::from_str(r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#);
    let rslt = create_tst(&conn, justj.unwrap());
    Ok(Value::String(format!("{:?} {:?}", js, meta)))
}


//struct Methd(AtomicUsize);
//impl RpcMethod<Meta> for Methd {
//    fn call(&self, params: Params, meta: Meta) -> BoxFuture<Value> {
//    }
//}
struct bye {

}
fn methd_bye(params: Params, meta: Meta) -> Result<Value> {
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

    let dbconn = establish_connection();

    let mut io = MetaIoHandler::with_middleware(MyMiddleware::default());

    io.add_method_with_meta("say_hello", &methd_hello);
    io.add_method_with_meta("say_bye", &methd_bye);
    io.add_method_with_meta("say_pg", &methd_pg);

    let _server = ServerBuilder::new(io)
//        .threads(3)
        .rest_api(RestApi::Unsecure)
        .meta_extractor(|req: &hyper::Request| {
            let methd =req.method().clone();
            let uri = req.uri().clone();
            let hdrs = req.headers().clone();
            let remote = req.remote_addr().clone();
            println!("{:?}", req);
//            let auth = Some(req.headers()
//                .get::<header::Host>().unwrap().hostname().to_string());
//            let auth = auth.map(|h| h.token.clone());
            Meta { methd, uri, hdrs, remote }
        })
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
        .start_http(&"0.0.0.0:3030".parse().unwrap())
        .expect("unable to start");

    _server.wait();
}

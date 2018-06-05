#![recursion_limit="128"]

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
mod api;
pub mod meta;
pub mod midware;
pub mod theapp;

use jsonrpc_core::*;
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi, hyper};
use midware::diesel::{MyMiddleware};
use theapp::DieselMidWare;
use utils::logger_factory;
use api::add_api;
use meta::Meta;

fn main() {
    let logger = logger_factory();
    let thepool = DieselMidWare::new(&logger);

    let mut io = MetaIoHandler::with_middleware(MyMiddleware::default());

    add_api(&mut io);

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

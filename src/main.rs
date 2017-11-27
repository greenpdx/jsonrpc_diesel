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

use std::thread;
use jsonrpc_core::*;
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi, MetaExtractor, hyper};
use self::hyper::{Method, Uri, HttpVersion, Headers, Body, header};
use jsonrpc_core::futures::Future;
use std::sync::atomic::{self, AtomicUsize};
use std::time::Instant;
use std::net::{SocketAddr, IpAddr};
use std::path::Path;
use midware::diesel::{DieselMidWare, MyMiddleware};
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

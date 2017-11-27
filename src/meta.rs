extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

use std::fmt;
use std::fmt::Formatter;
use jsonrpc_core::*;
use std::net::{SocketAddr, IpAddr};
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi, MetaExtractor, hyper};
use self::hyper::{Method, Uri, HttpVersion, Headers, Body, header};
use slog::Logger;
use midware::diesel::DieselMidWare;

#[derive(Clone,Default)]
pub struct Meta {
    pub remote: Option<SocketAddr>,
//    bob: (Method, Uri, HttpVersion, Headers, Body),
    pub methd: Method,
    pub uri: Uri,
    pub hdrs: Headers,
    pub dbpool: Option<DieselMidWare>,
    pub logger: Option<Logger>,
//    path: String,
}
impl Metadata for Meta {}

impl fmt::Debug for Meta {
    fn fmt(&self,f: &mut Formatter) -> fmt::Result {
        let pgstr = "PG".to_string();
        write!(f, "{:?} {:?} {:?} {:?} {:?}", self.remote, self.methd, self.uri, self.hdrs, pgstr)
    }
}

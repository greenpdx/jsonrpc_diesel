extern crate chrono;
extern crate serde_json;
extern crate serde;

use schema::tst1;
use std::time::SystemTime;
use jsonrpc_core::*;
use diesel::data_types::PgTimestamp;
use diesel::types::Timestamp;
use self::chrono::NaiveDateTime;
use serde::Serialize;
use std::fmt;

#[derive(Insertable,Debug)]
#[table_name="tst1"]
pub struct NewTst {
    pub methd: String,
    pub rpcid: i32,
}

#[derive(Queryable,Serialize, Debug)]
pub struct Tst1 {
    pub id: i32,
    pub ts: Option<NaiveDateTime>,
    pub methd: String,
    pub rpcid: i32,
}

impl fmt::Display for Tst1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ts = match self.ts {
            Some(x) => format!("{}", x),
            None => "None".to_string(),
        };
        write!(f, "{{id: {}, ts: {}, methd: {}}}", self.id, ts, self.methd )
    }
}
/*
impl<Tst1> fmt::Display for Vec<Tst1> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{id: {}, ts: {}, methd: {}}}", self.id, self.ts.unwrap(), self.methd )
    }
}
*/
pub type AryTst1 = Vec<Tst1>;

//impl Debug for Tst1 {

//}


use schema::tst1;
use std::time::SystemTime;
use jsonrpc_core::*;
use diesel::pg::types::sql_types::*;
use diesel::pg::types::sql_types::Json;

#[derive(Insertable,Debug)]
#[table_name="tst1"]
pub struct NewTst<'a> {
    pub js: &'a Json,
}

#[derive(Queryable)]
pub struct Tst1 {
    pub id: i32,
    pub ts: SystemTime,
    pub js: Json,
}

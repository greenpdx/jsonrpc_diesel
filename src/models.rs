extern crate chrono;
extern crate serde_json;
extern crate serde;

use schema::{item, sell};
use self::chrono::NaiveDateTime;
use std::fmt;
use self::chrono::*;

#[derive(Insertable,Debug,Default)]
#[table_name="item"]
pub struct NewItem {
    pub name: String,
    pub price: i32,
    pub desc: Option<String>,
    pub valid: i32,
}

#[derive(Queryable,Serialize, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub desc: Option<String>,
    pub valid: i32
}

#[derive(Insertable,Serialize,Debug)]
#[table_name="sell"]
pub struct NewSell {
    pub item_id: i32,
    pub sold: i32
}

#[derive(Queryable,Serialize, Debug)]
pub struct Sell {
    pub id: Option<i32>,
    pub item_id: i32,
    pub tz: i64,
    pub sold: Option<i32>
}

/*
impl fmt::Display for Vid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ts = match self. {
            Some(x) => format!("{}", x),
            None => "None".to_string(),
        };
        write!(f, "{{id: {}, ts: {}, methd: {}}}", self.id, ts, self.methd )
    }
}
*/

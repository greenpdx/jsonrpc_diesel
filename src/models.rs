extern crate chrono;
extern crate serde_json;
extern crate serde;

use schema::{item, sell, keymap};
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

#[derive(Queryable,QueryableByName,Serialize, Debug)]
#[table_name="item"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub desc: String,
    pub valid: i32
}

#[derive(Insertable,Serialize,Debug)]
#[table_name="sell"]
pub struct NewSell {
    pub item_id: i32,
    pub sold: Option<i32>
}

#[derive(Queryable,QueryableByName,Serialize, Debug)]
#[table_name="sell"]
pub struct Sell {
    pub id: i32,
    pub item_id: i32,
    pub tz: i64,
    pub sold: Option<i32>
}

#[derive(Queryable,QueryableByName,Serialize, Debug)]
#[table_name="keymap"]
pub struct KeyMap {
    pub id: i32,
    pub code: i16,
    pub key: i16,
    pub item_id: i32
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

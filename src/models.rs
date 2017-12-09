extern crate chrono;
extern crate serde_json;
extern crate serde;

use schema::{vids0, vidsinfo0};
use self::chrono::NaiveDateTime;
use std::fmt;

#[derive(Insertable,Debug,Default)]
#[table_name="vids0"]
pub struct NewVid {
    pub name: String,
    pub len: i64,
    pub duration: f32,
    pub bit_rate: i32,
    pub width: i16,
    pub height: i16,
}

#[derive(Queryable,Serialize, Debug)]
pub struct Vid {
    pub id: i32,
    pub name: String,
    pub len: i64,
    pub duration: f32,
    pub bit_rate: i32,
    pub width: Option<i16>,
    pub height: Option<i16>,

}

#[derive(Insertable,Queryable,Serialize,Debug)]
#[table_name="vidsinfo0"]
pub struct VidInfo {
    pub id: i32,
    pub access: Option<NaiveDateTime>,
    pub modify: Option<NaiveDateTime>,
    pub viewed: Option<i32>,
    pub rate: Option<i16>,
    pub quality: Option<i16>,
    pub accumtime: Option<i32>,
    pub fpath: Option<String>,
    pub hash: Option<String>,
}

#[derive(Queryable,Serialize, Debug)]
pub struct VidFull {
    pub id: i32,
    pub name: String,
    pub len: i64,
    pub duration: f32,
    pub bit_rate: i32,
    pub width: Option<i16>,
    pub height: Option<i16>,
    pub access: Option<NaiveDateTime>,
    pub modify: Option<NaiveDateTime>,
    pub viewed: Option<i32>,
    pub rate: Option<i16>,
    pub quality: Option<i16>,
    pub accumtime: Option<i32>,
    pub fpath: Option<String>,
    pub hash: Option<String>,

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

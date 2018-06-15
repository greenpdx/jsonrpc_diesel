//extern crate diesel;

use std::io::Read;
// use std::fs;
use std::fs::File;
use std::mem;
use std::sync::mpsc;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use midware::diesel::{MyMiddleware};
use chrono::prelude::*;
//use r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel;
//use r2d2_sqlite::SqliteConnectionManager;
use models::*;
use schema::*;
use std::collections::HashMap;
use std::char;

//use midware::sold::{Sales};
use theapp::DieselMidWare;

struct evt {
    time: u32,
    ktype: u16,
    code: u16,
    val:  i32,
    dummy: [u8;4]
}
/*
 * 4 4 keyscan
 * 1 keycode 1/0 press/release
 * 0 0 0
 */

 #[derive(Clone,Debug,Default,Copy,Serialize)]
 pub struct Sales {
     pub veg: u16,
     pub beef: u16,
     pub pork: u16,
     pub chik: u16,
     pub spcl: u16,
     pub soup: u16,
     pub pie: u16
 }

 impl Sales {
     fn new() -> Sales {
         Sales {
             veg: 0,
             beef: 0,
             pork: 0,
             chik: 0,
             spcl: 0,
             soup: 0,
             pie: 0
         }
     }
 }
pub fn xlat(code: u16) -> String {
     let ts = Local::now().timestamp();
     println!("S{:?}.", code);
     let key = match code {
         69 => { //numlock
             "NumLock"
         },
         98 => { // div /
             "div"
         },
         55 => { // mul *
             "mul"
         },
         14 => { // BS <-
             "BS"
         },
         71 => { // 7
             "7"
         },
         72 => { // 8
             "8"
         },
         73 => { // 9
             "9"
         },
         74 => { // minus -
             "minus"
         },
         75 => { // 4
             //self.chik += 1;
             "4"
         },
         76 => { // 5
             "5"
         },
         77 => { // 6
             "6"
         },
         78 => { // plus +
             "plus"
         },
         79 => { // 1
             //self.veg += 1;
             "1"
         },
         80 => { // 2
             //self.beef += 1;
             "2"
         },
         81 => { // 3
             //self.pork += 1;
             "3"
         },
         82 => { // 0
             "0"
         },
         83 => { // dot .
             "dot"
         },
         96 => { // enter
             "enter"
         },
         _ => {  // all else
             "all else"
         }
     };
     key.to_string()
}

const local: bool = true;

// read in key-item table
// create hashmap
pub fn capkey(pool: &r2d2::Pool<ConnectionManager<SqliteConnection>>, tx: mpsc::Sender<i32>) {
//    let mut input = String::new();
    use schema::sell::dsl::*;
    let mut buf0 = [0;16];
    let mut f0 = File::open("/dev/input/event0").expect("Can not open kbd");
    //let mut vbuf = Vec::new();
    let md = f0.metadata();
    let done = false;
    println!("Run keycap!\n {:?}\n {:#X?}", f0, md);
    let conn = pool.get().unwrap();

    let rslt: Vec<KeyMap> = diesel::sql_query("SELECT * FROM keymap")
        .load(&*conn).expect("Bad get keymap");

    let mut keyfind = HashMap::new();

    for key in rslt {
        let find = if false {key.key} else {key.code};
        keyfind.insert(find, key.item_id);
    };

    println!("{:?}", &keyfind );
//    Ok(Value::String(format!("{:?}", &r)))
//    Ok(Value::String(r.to_string()))

    while !done {
        let _sz = f0.read(&mut buf0).expect("NO read KBD");
        let tim: u64 = unsafe {
            mem::transmute([buf0[5],buf0[4],buf0[7],buf0[6],buf0[1],buf0[0],buf0[3],buf0[2]])
        };
//        let mut ktype: i16 = 1;
//        let mut code: i16 = 0;
//        let mut val: i16 = 1;
        
        let code: i16 = buf0[10].into();
        let ktype: i16 = buf0[8].into();
	let val: i16 = buf0[12].into();
        println!("Start Loop {:?} {:?}", code, buf0);
        if ktype == 1 && val == 1 {
            let itm_id = keyfind.get(&code).unwrap_or(&0);
            if *itm_id == 0 {
                continue;
            }
            let row = diesel::insert_into(sell).values((
                item_id.eq(itm_id)))
                .execute(&*conn);


            println!("C{:?} I{:?} V{:?}\n", code, itm_id, val);
            tx.send(itm_id.clone());
        }
        //println!("{:?} {:?} {:#X?} {:#X?} {:#X?}",sz, tim/1000000, ktype, code, val);
    }
}

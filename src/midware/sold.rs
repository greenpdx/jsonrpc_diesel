
use slog::Logger;
use dotenv::dotenv;
use std::env;
use std::sync::atomic::{self, AtomicUsize};
use jsonrpc_core::*;
use jsonrpc_core::futures::Future;
use meta::Meta;
use std::time::Instant;
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::fmt;
use std::fmt::Formatter;
use keycap::{Sales, capkey};
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, RwLock, RwLockReadGuard};
/*
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
    pub fn sale(&mut self, code: u16) -> String {
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
                self.chik += 1;
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
                self.veg += 1;
                "1"
            },
            80 => { // 2
                self.beef += 1;
                "2"
            },
            81 => { // 3
                self.pork += 1;
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
}*/
/*
impl Serialize for Sales {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {

    }
}
*/

//#[derive(Clone,Copy)]
pub struct SoldMidWare {

    pub sales: Sales,
    pub tsales: Arc<RwLock<Sales>>,
    fil: File
}
impl SoldMidWare {
	pub fn new (logger: &Logger) -> SoldMidWare{
		let logger = logger.new(o!("module" => "SoldMidWare"));
        dotenv().ok();

        let sales = Sales::default();
        let mut tsales = Arc::new(RwLock::new(sales));

        let (tx, _rx): (mpsc::Sender<u32>, mpsc::Receiver<u32>) = mpsc::channel();
        let mut tsale = tsales.clone();
        let _chld = thread::spawn(move || capkey(&tsale,tx));

        //let rsales = sales.read().expect("READ tsales");
        let logfile = env::var("LOGFILE").expect("edit .env for log file name");

        let f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(logfile)
            .unwrap();

		info!(logger, "Sales created");
        println!("NS{:?}", sales);
		SoldMidWare {sales: sales, tsales: tsales, fil: f}
	}
//    pub fn get(&self) -> Option<DieselConnection> {
//        let ref pool = self.pool;
//        Some(pool.unwrap().get().unwrap())
//    }
    pub fn sale (&mut self, key: u16) {
        let ts = Local::now().timestamp();
        let mut f = &self.fil;
        write!(f, "{:?} {:?}", key, ts);
    }
}

impl Clone for SoldMidWare {
    fn clone(&self) -> Self {
        SoldMidWare {
            sales: self.sales.clone(),
            tsales: self.tsales.clone(),
            fil: self.fil.try_clone().unwrap()
        }
    }
}

impl fmt::Debug for SoldMidWare {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        println!("{:?}", self.tsales);
        write!(f, "{:?} {:?} {:?} {:?}\n{:?}",
               self.sales.veg,
               self.sales.beef,
               self.sales.pork,
               self.sales.chik,
               self.tsales)
    }
}

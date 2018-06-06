use jsonrpc_core::*;
use meta::Meta;
use diesel::sqlite::SqliteConnection;
use diesel;
use schema;
use models::{NewItem, Item};
use diesel::prelude::*;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone )]
struct RawParams {
    pub table: String,
    pub intv: i32,
}

pub fn methd_add_item(params: Params, meta: Meta) -> Result<Value> {
    //use schema::vidsinfo0::dsl::*;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let p: RawParams = params.parse().unwrap();
    let row = "";
    //let intv = p.intv; // .parse::<i32>().unwrap();
    //let row = diesel::update(vidsinfo0.filter(id.eq(1)))
    //    .set(viewed.eq(intv))
    //    .get_result::<VidInfo>(&*conn);
//    let qry = sql_function!( "vids0", Vid, (a: None) -> Vec<i64>);
//    println!("INS {:?} {:?}", js, meta);
//    let vid = NewVid::default();
//    let _rslt = create_vid(&*conn, &vid);
//    let vids = diesel::expression::sql::<Vid>(&p.intv);
//    let out = vids.get_results(&*conn);
println!("INS {:?}", row);

//    let out = vids.load(&*conn).expect("bad");

    Ok(Value::String(format!("{:?}", "")))
}

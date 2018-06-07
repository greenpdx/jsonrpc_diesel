use jsonrpc_core::*;
use meta::Meta;
use diesel::sqlite::SqliteConnection;
use diesel;
use schema;
use models::{NewItem, Item};
use diesel::prelude::*;
use std::any::Any;
use diesel::insert_into;
use std::ops::Deref;


#[derive(Serialize, Deserialize, Debug, Clone )]
struct AddParams {
    pub name: String,
    pub price: i32,
    pub desc: String
}

#[derive(Serialize, Deserialize, Debug, Clone )]
struct GetParams {
    pub search: Option<String>
}

pub fn methd_get_items(params: Params, meta: Meta) -> Result<Value> {
    use schema::item::dsl::*;
    use diesel::sql_query;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();
    println!("PARAMS {:?}", params );
    let p: GetParams = params.parse().unwrap_or(GetParams{search:None});
    println!("RAWP {:?}", &p );

    //let rslt = item.load::<Item>(&*conn).expect("tsts");
    //let rslt = item.filter(valid.eq(true))
    //    .load::<Item>(&*conn)
    //    .expect("bas get item");
    let rslt: Vec<Item> = diesel::sql_query("SELECT * FROM item WHERE valid = 1")
        .load(&*conn).expect("Bad get item");
    let r = json!(rslt);
//    Ok(Value::String(format!("{:?}", &r)))
    Ok(Value::String(r.to_string()))
}

pub fn methd_add_item(params: Params, meta: Meta) -> Result<Value> {
    use schema::item::dsl::*;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();
    println!("PARAMS {:?}", params );

    let p: AddParams = params.parse()?;
    println!("RAWP {:?}", &p );
    let row = "";
    //let intv = p.intv; // .parse::<i32>().unwrap();
    let row = insert_into(item).values((
        name.eq(p.name),
        price.eq(p.price),
        desc.eq(p.desc)))
        .execute(&*conn);

    //let row = diesel::insert(item.filter(id.eq(1)))
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

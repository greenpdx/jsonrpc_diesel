use jsonrpc_core::*;
use meta::Meta;
use diesel::sqlite::SqliteConnection;
use diesel;
use schema;
use models::{NewItem, Item, Sell};
use diesel::prelude::*;
use std::any::Any;
use diesel::insert_into;
use std::ops::Deref;
use diesel::dsl::*;
use diesel::sql_types::Integer;

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

use schema::sell;
#[derive(QueryableByName,Serialize, Deserialize, Debug, Clone,PartialEq )]
//#[table_name="sell"]
struct ItemCount {
    #[sql_type = "Integer"]
    count: i32,
    #[sql_type = "Integer"]
    item_id: i32,
}
type DB = diesel::sqlite::Sqlite;
impl Queryable<sell::SqlType, DB> for ItemCount {
    type Row = (i32, i32, i64, Option<i32>);

    fn build(row: Self::Row) -> Self {
        ItemCount {
            count: row.0,
            item_id: row.1,
        }
    }
}

pub fn methd_get_sold(_params: Params, meta: Meta) -> Result<Value> {
    use schema::sell::dsl::*;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let setp = diesel::sql_query("SELECT count(item_id) as count, item_id FROM sell GROUP BY item_id");
    let rcnt: Vec<ItemCount> = setp.load(&*conn).expect("count bad");

    println!("{:?}", rcnt );
    let r = json!(rcnt);

    Ok(Value::String(r.to_string()))
}

pub fn methd_get_items(params: Params, meta: Meta) -> Result<Value> {
    use schema::item::dsl::*;
    use schema::sell::dsl::*;
    use diesel::sql_query;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();
    println!("PARAMS {:?}", params );
    let p: GetParams = params.parse().unwrap_or(GetParams{search:None});
    println!("RAWP {:?}", &p );

    let rslt: Vec<Item> = diesel::sql_query("SELECT * FROM item WHERE valid = 1")
        .load(&*conn).expect("Bad get item");

    //let setp = diesel::sql_query("SELECT count(item_id) as count, item_id FROM sell GROUP BY item_id");
    //let rcnt: Vec<ItemCount> = setp.load(&*conn).expect("count bad");
    //println!("{:?}", rcnt );

    let r = json!(rslt).to_string();
    println!("GIS{:?}", &r );
    Ok(Value::String(r))
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

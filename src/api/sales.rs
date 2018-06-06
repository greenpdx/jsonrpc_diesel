use jsonrpc_core::*;
use meta::Meta;
use schema;
use diesel::prelude::*;

pub fn methd_get_sales(_params: Params, meta: Meta) -> Result<Value> {
    use self::schema::sell::dsl::*;
    use models::Sell;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let rslt = sell.filter(id.ne(0))
        .load::<Sell>(&*conn)
        .expect("Error");
    let r = json!(&rslt);
//    println!("{:?}", r);
//    for itm in rslt {
//        let s = serde_json::to_string(&itm).unwrap();
//        println!("{} {} {}", itm.id, s, itm);
//    }
    Ok(Value::String(format!("{}", &r)))
}

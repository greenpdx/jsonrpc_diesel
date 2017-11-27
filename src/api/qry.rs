use jsonrpc_core::*;
use meta::Meta;
use schema;
use diesel::prelude::*;

pub fn methd_qry(_params: Params, meta: Meta) -> Result<Value> {
    use self::schema::tst1::dsl::*;
    use models::Tst1;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let rslt = tst1.filter(id.ne(0))
        .load::<Tst1>(&*conn)
        .expect("Error");
    let r = json!(&rslt);
//    println!("{:?}", r);
//    for itm in rslt {
//        let s = serde_json::to_string(&itm).unwrap();
//        println!("{} {} {}", itm.id, s, itm);
//    }
    Ok(Value::String(format!("{}", &r)))
}

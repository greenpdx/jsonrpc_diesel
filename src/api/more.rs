use jsonrpc_core::*;
use meta::Meta;
use schema;
use diesel::prelude::*;
use diesel::*;
use schema::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SqlQry {
    pub table: String,
    pub sql: String,
}

pub fn methd_more(params: Params, meta: Meta) -> Result<Value> {
    use self::schema::vids::dsl::*;
    use models::VidInfo;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    //let parm: &SqlQry = &params.parse().unwrap();
    let rslt: i64 = vids.count()
        .get_result(&*conn)
        .expect("Error");
    let r = json!(&rslt);
//    println!("{:?}", r);
//    for itm in rslt {
//        let s = serde_json::to_string(&itm).unwrap();
//        println!("{} {} {}", itm.id, s, itm);
//    }
    Ok(Value::String(format!("{}", &r)))
}

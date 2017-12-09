use jsonrpc_core::*;
use meta::Meta;
use schema;
use diesel::prelude::*;
use diesel::Expression;

#[derive(Serialize, Deserialize, Debug )]
struct Order {
    pub field: String,
    pub expr: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bye {
    pub start: i64,
    pub limit: i64,
    pub sort: Option<Vec<Order>>
}

pub fn methd_bye(params: Params, meta: Meta) -> Result<Value> {
    use self::schema::vidfull::dsl::*;
    use models::VidFull;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();
    println!("{:?}", &params);
    let parm: &Bye = &params.parse().unwrap();
    println!("BYE {:?}", parm);
//    let sort = &parm.sort[0];
//    let fld = &sort.field;
/*    let ordering: Box<BoxableExpression<fullvid, DB, SqlType=()>> =
        match sort.expr.as_ref() {
            "desc" => fld.desc(),
            "asc" => fld.asc(),
        }; */
    let rslt = vidfull.filter(id.ne(0))
//        .order(())
        .offset(parm.start)
        .limit(parm.limit)
        .load::<VidFull>(&*conn)
        .expect("Error");
    let r = json!(&rslt);
//    println!("{:?}", r);
//    for itm in rslt {
//        let s = serde_json::to_string(&itm).unwrap();
//        println!("{} {} {}", itm.id, s, itm);
//    }
    Ok(Value::String(format!("{}", &r)))
}

use jsonrpc_core::*;
use meta::Meta;
use diesel::pg::PgConnection;
use diesel;
use schema;
use models;
use diesel::prelude::*;

fn create_tst<'a>(conn: &PgConnection, cmd: String, rpcid: i32)  {
    use self::schema::tst1;
    let new_tst = &models::NewTst {
        methd: cmd,
        rpcid: rpcid,
    };
//    let tst: models::Tst1 = diesel::insert_into(tst1::table)
//        .values(cmd)
//        .get_results(conn)
//        .expect("Error");
    let _tst = diesel::insert(new_tst)
        .into(tst1::table)
        .execute(conn)
        .expect("Error");

//    tst
}

pub fn methd_ins(params: Params, meta: Meta) -> Result<Value> {
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let js: [i32;2] = params.parse().unwrap();
//    println!("INS {:?} {:?}", js, meta);
    let _rslt = create_tst(&*conn, "say_ins".to_string(), 1);
    Ok(Value::String(format!("{:?}", js)))
}

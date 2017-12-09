use jsonrpc_core::*;
use meta::Meta;
use diesel::pg::PgConnection;
use diesel;
use schema;
use models::{NewVid, Vid, VidInfo, VidFull};
use diesel::prelude::*;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone )]
struct RawParams {
    id: i32,
    intv: i32
}

fn create_vid<'a>(conn: &PgConnection, vid: &NewVid)  {
    use self::schema::vids0;
    let new_tst = &NewVid {
        name: vid.name.clone(),
        len: vid.len,
        duration: vid.duration,
        bit_rate: vid.bit_rate,
        width: vid.width,
        height: vid.height,
    };
//    let tst: models::Tst1 = diesel::insert_into(tst1::table)
//        .values(cmd)
//        .get_results(conn)
//        .expect("Error");
    let _tst = diesel::insert(new_tst)
        .into(vids0::table)
        .execute(conn)
        .expect("Error");

//    tst
}

pub fn mod_val(params: Params, meta: Meta) -> Result<Value> {
    use schema::vidsinfo0::dsl::*;
    let pool = meta.dbpool;
    let conn = pool.unwrap().pool.get().unwrap();

    let p: RawParams = params.parse().unwrap();

    // numeric_expr!(<vidsinfo0>::viewed);

    let intv = p.id; // .parse::<i32>().unwrap();
    let row = diesel::update(vidsinfo0.filter(id.eq(p.id)))
        .set(viewed.eq(viewed + p.intv))
        .get_result::<VidInfo>(&*conn)
        .expect("adad");

    let js = json!(&row);
    println!("MODVAL {:?}",js);
//    let qry = sql_function!( "vids0", Vid, (a: None) -> Vec<i64>);
//    println!("INS {:?} {:?}", js, meta);
//    let vid = NewVid::default();
//    let _rslt = create_vid(&*conn, &vid);
//    let vids = diesel::expression::sql::<Vid>(&p.intv);
//    let out = vids.get_results(&*conn);
println!("INS {:?}", js);

//    let out = vids.load(&*conn).expect("bad");

    Ok(Value::String(format!("{}", &js)))
}

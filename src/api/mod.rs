pub mod hello;
pub mod getsales;
//pub mod ins;
//pub mod qry;
//pub mod more;
//pub mod modval;

use jsonrpc_core::*;
use api::hello::methd_hello;
use api::getsales::*;
//use api::ins::methd_ins;
//use api::qry::methd_qry;
//use api::more::methd_more;
//use api::modval::mod_val;
use meta::Meta;
use midware::diesel::MyMiddleware;

pub fn add_api(io: &mut MetaIoHandler<Meta, MyMiddleware>) {
    io.add_method_with_meta("say_hello", &methd_hello);
    io.add_method_with_meta("get_sales", &methd_get_sales);
//    io.add_method_with_meta("say_ins", &methd_ins);
//    io.add_method_with_meta("say_qry", &methd_qry);
//    io.add_method_with_meta("say_more", &methd_more);
//    io.add_method_with_meta("mod_val", &mod_val);
}

pub mod hello;
pub mod sales;
pub mod item;
//pub mod qry;
//pub mod more;
//pub mod modval;

use jsonrpc_core::*;
use api::hello::methd_hello;
use api::sales::*;
use api::item::*;
//use api::ins::methd_ins;
//use api::qry::methd_qry;
//use api::more::methd_more;
//use api::modval::mod_val;
use meta::Meta;
use midware::diesel::MyMiddleware;

pub fn add_api(io: &mut MetaIoHandler<Meta, MyMiddleware>) {
    io.add_method_with_meta("say_hello", &methd_hello);
    io.add_method_with_meta("get_sales", &methd_get_sales);
    io.add_method_with_meta("item_add", &methd_add_item);
//    io.add_method_with_meta("say_qry", &methd_qry);
//    io.add_method_with_meta("say_more", &methd_more);
//    io.add_method_with_meta("mod_val", &mod_val);
}

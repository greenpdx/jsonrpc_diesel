use jsonrpc_core::*;
use meta::Meta;

pub fn methd_hello(_parm: Params, meta: Meta) -> Result<Value> {
    let remote = meta.remote.unwrap(); // .unwrap_or_else(String::new);
    println!("Hello {:?} {:?}", _parm, remote );
    let ipaddr = remote.ip();
    let port = remote.port();
    let ans = format!("{}", remote);
    Ok(Value::String(ans))
}

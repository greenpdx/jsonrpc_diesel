use jsonrpc_core::*;
use meta::Meta;

pub fn methd_bye(params: Params, meta: Meta) -> Result<Value> {
    let js: [i32;2] = params.parse().unwrap();
    Ok(Value::String(format!("{:?}", js)))
}

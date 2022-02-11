mod dex_bindings;

use crate::dex_bindings::{Dex, DexData, PoolPair, PoolPrice, SwapResult, TokenAmount};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use wit_bindgen_wasmtime::anyhow::{anyhow, bail, Result};
use wit_bindgen_wasmtime::wasmtime::*;

#[no_mangle]
pub extern "C" fn ainrt_execute_dex_swap(
    dex_module_file_path: *const c_char,
    poolpair: *mut PoolPair,
    token_in: &TokenAmount,
    max_price: &PoolPrice,
    post_bayfront_gardens: bool,
) -> i64 {
    let pp = unsafe { *poolpair.clone() };
    let tk_in = unsafe { token_in.clone() };
    let mp = unsafe { max_price.clone() };
    let dex_module_file_path = unsafe { CStr::from_ptr(dex_module_file_path) }
        .to_str()
        .unwrap();

    match dex_swap(dex_module_file_path, pp, tk_in, mp, post_bayfront_gardens) {
        Ok(res) => {
            unsafe { *poolpair = res.pool_pair }
            res.slop_swap_result
        }
        Err(_) => 0,
    }
}

struct Context {
    pub wasi: WasiCtx,
}

fn dex_swap<P: AsRef<Path>>(
    path: P,
    poolpair: PoolPair,
    token_in: TokenAmount,
    max_price: PoolPrice,
    post_bayfront_gardens: bool,
) -> Result<SwapResult> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let dex_module = Module::from_file(&engine, path)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(&engine, wasi);
    let (dex, _instance) = Dex::instantiate(&mut store, &dex_module, &mut linker)?;
    let result = dex.swap(
        &mut store,
        poolpair,
        token_in,
        max_price,
        post_bayfront_gardens,
    )?;
    result.map_err(|e| anyhow!(format!("{:?}", e)))
}

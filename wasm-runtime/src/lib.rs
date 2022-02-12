mod dex_bindings;

use crate::dex_bindings::{Dex, DexData, PoolPair, PoolPrice, SwapResult, TokenAmount};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::sync::Arc;
use dashmap::DashMap;
use lazy_static::lazy_static;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use wit_bindgen_wasmtime::anyhow::{anyhow, bail, Result};
use wit_bindgen_wasmtime::wasmtime::*;

lazy_static! {
    static ref MODULEMAP: Arc<DashMap<String, Instance>> = {
        let mut m = DashMap::new();
        Arc::new(m)
    };
    static ref STOREMAP: Arc<DashMap<String, Store<WasiCtx>>> = {
        let mut m = DashMap::new();
        Arc::new(m)
    };
}

#[no_mangle]
pub extern "C" fn ainrt_register_dex_module(
    dex_module_file_path: *const c_char,
) -> i32 {
    let dex_module_file_path = unsafe { CStr::from_ptr(dex_module_file_path) }
        .to_str()
        .unwrap();
    match register_dex_module(dex_module_file_path) {
        Ok(_) => {
            1
        }
        Err(_) => {
            0
        }
    }

}

fn register_dex_module<P : AsRef<Path>>(
   path: P,
) -> Result<()>{
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let dex_module = Module::from_file(&engine, path)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(&engine, wasi);
    // linker.instantiate()
    let (_, instance) = Dex::instantiate(&mut store, &dex_module, &mut linker)?;
    MODULEMAP.insert("dex".to_string(), instance);
    STOREMAP.insert("dex".to_string(), store);
    Ok(())
}

#[no_mangle]
pub extern "C" fn ainrt_execute_dex_swap(
    poolpair: *mut PoolPair,
    token_in: &TokenAmount,
    max_price: &PoolPrice,
    post_bayfront_gardens: bool,
) -> i64 {
    let pp = unsafe { *poolpair.clone() };
    let tk_in = unsafe { token_in.clone() };
    let mp = unsafe { max_price.clone() };

    match dex_swap(pp, tk_in, mp, post_bayfront_gardens) {
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

fn dex_swap(
    poolpair: PoolPair,
    token_in: TokenAmount,
    max_price: PoolPrice,
    post_bayfront_gardens: bool,
) -> Result<SwapResult> {
    let dex = Dex::new(STOREMAP.get_mut("dex").unwrap().value_mut(), MODULEMAP.get("dex").unwrap().value())?;
    let result = dex.swap(
        &mut STOREMAP.get_mut("dex").unwrap().value_mut(),
        poolpair,
        token_in,
        max_price,
        post_bayfront_gardens,
    )?;
    result.map_err(|e| anyhow!(format!("{:?}", e)))
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Instant;
    use crate::{dex_swap, PoolPair, PoolPrice, register_dex_module, TokenAmount};
    const COIN: i64 = 100000000;
    #[test]
    fn text_swap() {


        let gold = 1;
        let silver = 2;

        let mut pool_pair = PoolPair {
            token_a: gold,
            token_b: silver,
            commission: (0.1 as f64 * COIN as f64) as u32,
            reserve_a: 200 * COIN,
            reserve_b: 1000 * COIN,
            total_liquidity: 1000 * COIN,
            block_commission_a: 0,
            block_commission_b: 0
        };
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.pop();
        println!("{:?}", d);
        let path = d.join("pkg/modules-wasm/dex.wasm");
        register_dex_module(path).unwrap();

        let token_in = TokenAmount {
            token_id: silver,
            amount: 10 * COIN
        };
        let max_price = PoolPrice {
            integer: 100 * COIN,
            fraction: 0
        };
        let instant = Instant::now();
        for i in 1..21 {
            let result = dex_swap(pool_pair.clone(), token_in.clone(), max_price.clone(), true).unwrap();
            println!("Result {}: {:#?}",i, result.slop_swap_result as f64 / COIN as f64);
            pool_pair = result.pool_pair;
        }
        println!("21 swaps took {}ms", instant.elapsed().as_millis())
    }
}
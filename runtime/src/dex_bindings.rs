#[allow(unused_imports)]
use wit_bindgen_wasmtime::{wasmtime, anyhow};
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error{
    RuntimeError,
    NotFoundError,
    InvalidInput,
    LackOfLiquidity,
    PriceHigherThanIndex,
    PoolReserveOverflow,
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RuntimeError => {
                f.debug_tuple("Error::RuntimeError").finish()
            }
            Error::NotFoundError => {
                f.debug_tuple("Error::NotFoundError").finish()
            }
            Error::InvalidInput => {
                f.debug_tuple("Error::InvalidInput").finish()
            }
            Error::LackOfLiquidity => {
                f.debug_tuple("Error::LackOfLiquidity").finish()
            }
            Error::PriceHigherThanIndex => {
                f.debug_tuple("Error::PriceHigherThanIndex").finish()
            }
            Error::PoolReserveOverflow => {
                f.debug_tuple("Error::PoolReserveOverflow").finish()
            }
        }
    }
}
pub type DctId = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TokenAmount {
    pub token_id: DctId,
    pub amount: i64,
}
impl std::fmt::Debug for TokenAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenAmount").field("token-id", &self.token_id).field("amount", &self.amount).finish()}
}
impl wit_bindgen_wasmtime::Endian for TokenAmount {
    fn into_le(self) -> Self {
        Self {
            token_id: self.token_id.into_le(),
            amount: self.amount.into_le(),
        }
    }
    fn from_le(self) -> Self {
        Self {
            token_id: self.token_id.from_le(),
            amount: self.amount.from_le(),
        }
    }
}
unsafe impl wit_bindgen_wasmtime::AllBytesValid for TokenAmount {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PoolPrice {
    pub integer: i64,
    pub fraction: i64,
}
impl std::fmt::Debug for PoolPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPrice").field("integer", &self.integer).field("fraction", &self.fraction).finish()}
}
impl wit_bindgen_wasmtime::Endian for PoolPrice {
    fn into_le(self) -> Self {
        Self {
            integer: self.integer.into_le(),
            fraction: self.fraction.into_le(),
        }
    }
    fn from_le(self) -> Self {
        Self {
            integer: self.integer.from_le(),
            fraction: self.fraction.from_le(),
        }
    }
}
unsafe impl wit_bindgen_wasmtime::AllBytesValid for PoolPrice {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PoolPair {
    pub token_a: DctId,
    pub token_b: DctId,
    pub commission: DctId,
    pub reserve_a: i64,
    pub reserve_b: i64,
    pub total_liquidity: i64,
    pub block_commission_a: i64,
    pub block_commission_b: i64,
}
impl std::fmt::Debug for PoolPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPair").field("token-a", &self.token_a).field("token-b", &self.token_b).field("commission", &self.commission).field("reserve-a", &self.reserve_a).field("reserve-b", &self.reserve_b).field("total-liquidity", &self.total_liquidity).field("block-commission-a", &self.block_commission_a).field("block-commission-b", &self.block_commission_b).finish()}
}
impl wit_bindgen_wasmtime::Endian for PoolPair {
    fn into_le(self) -> Self {
        Self {
            token_a: self.token_a.into_le(),
            token_b: self.token_b.into_le(),
            commission: self.commission.into_le(),
            reserve_a: self.reserve_a.into_le(),
            reserve_b: self.reserve_b.into_le(),
            total_liquidity: self.total_liquidity.into_le(),
            block_commission_a: self.block_commission_a.into_le(),
            block_commission_b: self.block_commission_b.into_le(),
        }
    }
    fn from_le(self) -> Self {
        Self {
            token_a: self.token_a.from_le(),
            token_b: self.token_b.from_le(),
            commission: self.commission.from_le(),
            reserve_a: self.reserve_a.from_le(),
            reserve_b: self.reserve_b.from_le(),
            total_liquidity: self.total_liquidity.from_le(),
            block_commission_a: self.block_commission_a.from_le(),
            block_commission_b: self.block_commission_b.from_le(),
        }
    }
}
unsafe impl wit_bindgen_wasmtime::AllBytesValid for PoolPair {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapResult {
    pub pool_pair: PoolPair,
    pub slop_swap_result: i64,
}
impl std::fmt::Debug for SwapResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwapResult").field("pool-pair", &self.pool_pair).field("slop-swap-result", &self.slop_swap_result).finish()}
}
impl wit_bindgen_wasmtime::Endian for SwapResult {
    fn into_le(self) -> Self {
        Self {
            pool_pair: self.pool_pair.into_le(),
            slop_swap_result: self.slop_swap_result.into_le(),
        }
    }
    fn from_le(self) -> Self {
        Self {
            pool_pair: self.pool_pair.from_le(),
            slop_swap_result: self.slop_swap_result.from_le(),
        }
    }
}
unsafe impl wit_bindgen_wasmtime::AllBytesValid for SwapResult {}

/// Auxiliary data associated with the wasm exports.
///
/// This is required to be stored within the data of a
/// `Store<T>` itself so lifting/lowering state can be managed
/// when translating between the host and wasm.
#[derive(Default)]
pub struct DexData {
}
pub struct Dex<T> {
    get_state: Box<dyn Fn(&mut T) -> &mut DexData + Send + Sync>,
    memory: wasmtime::Memory,
    swap: wasmtime::TypedFunc<(i32,i32,i32,i64,i64,i64,i64,i64,i32,i64,i64,i64,i32,), (i32,)>,
}
impl<T> Dex<T> {
    #[allow(unused_variables)]

    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `linker` provided.
    ///
    /// The `get_state` closure is required to access the
    /// auxiliary data necessary for these wasm exports from
    /// the general store's state.
    pub fn add_to_linker(
        linker: &mut wasmtime::Linker<T>,
        get_state: impl Fn(&mut T) -> &mut DexData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `linker` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_linker` beforehand. This function will
    /// instantiate the `module` otherwise using `linker`, and
    /// both an instance of this structure and the underlying
    /// `wasmtime::Instance` will be returned.
    ///
    /// The `get_state` parameter is used to access the
    /// auxiliary state necessary for these wasm exports from
    /// the general store state `T`.
    pub fn instantiate(
        mut store: impl wasmtime::AsContextMut<Data = T>,
        module: &wasmtime::Module,
        linker: &mut wasmtime::Linker<T>,
        get_state: impl Fn(&mut T) -> &mut DexData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<(Self, wasmtime::Instance)> {
        Self::add_to_linker(linker, get_state)?;
        let instance = linker.instantiate(&mut store, module)?;
        Ok((Self::new(store, &instance,get_state)?, instance))
    }

    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
        mut store: impl wasmtime::AsContextMut<Data = T>,
        instance: &wasmtime::Instance,
        get_state: impl Fn(&mut T) -> &mut DexData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<Self> {
        let mut store = store.as_context_mut();
        let memory= instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| {
                anyhow::anyhow!("`memory` export not a memory")
            })?
            ;
        let swap= instance.get_typed_func::<(i32,i32,i32,i64,i64,i64,i64,i64,i32,i64,i64,i64,i32,), (i32,), _>(&mut store, "swap")?;
        Ok(Dex{
            memory,
            swap,
            get_state: Box::new(get_state),

        })
    }
    pub fn swap(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,poolpair: PoolPair,token_in: TokenAmount,max_price: PoolPrice,post_bayfront_gardens: bool,)-> Result<Result<SwapResult,Error>, wasmtime::Trap> {
        let memory = &self.memory;
        let PoolPair{ token_a:token_a0, token_b:token_b0, commission:commission0, reserve_a:reserve_a0, reserve_b:reserve_b0, total_liquidity:total_liquidity0, block_commission_a:block_commission_a0, block_commission_b:block_commission_b0, } = poolpair;
        let TokenAmount{ token_id:token_id1, amount:amount1, } = token_in;
        let PoolPrice{ integer:integer2, fraction:fraction2, } = max_price;
        let result3 = match post_bayfront_gardens{
            false => { 0i32}
            true => { 1i32}
        };
        let (result4_0,) = self.swap.call(&mut caller, (wit_bindgen_wasmtime::rt::as_i32(token_a0), wit_bindgen_wasmtime::rt::as_i32(token_b0), wit_bindgen_wasmtime::rt::as_i32(commission0), wit_bindgen_wasmtime::rt::as_i64(reserve_a0), wit_bindgen_wasmtime::rt::as_i64(reserve_b0), wit_bindgen_wasmtime::rt::as_i64(total_liquidity0), wit_bindgen_wasmtime::rt::as_i64(block_commission_a0), wit_bindgen_wasmtime::rt::as_i64(block_commission_b0), wit_bindgen_wasmtime::rt::as_i32(token_id1), wit_bindgen_wasmtime::rt::as_i64(amount1), wit_bindgen_wasmtime::rt::as_i64(integer2), wit_bindgen_wasmtime::rt::as_i64(fraction2), result3, ))?;
        let load5 = memory.data_mut(&mut caller).load::<i32>(result4_0 + 0)?;
        let load6 = memory.data_mut(&mut caller).load::<i32>(result4_0 + 8)?;
        let load7 = memory.data_mut(&mut caller).load::<i32>(result4_0 + 16)?;
        let load8 = memory.data_mut(&mut caller).load::<i32>(result4_0 + 24)?;
        let load9 = memory.data_mut(&mut caller).load::<i64>(result4_0 + 32)?;
        let load10 = memory.data_mut(&mut caller).load::<i64>(result4_0 + 40)?;
        let load11 = memory.data_mut(&mut caller).load::<i64>(result4_0 + 48)?;
        let load12 = memory.data_mut(&mut caller).load::<i64>(result4_0 + 56)?;
        let load13 = memory.data_mut(&mut caller).load::<i64>(result4_0 + 64)?;
        let load14 = memory.data_mut(&mut caller).load::<i64>(result4_0 + 72)?;
        Ok(match load5 {
            0 => Ok(SwapResult{pool_pair:PoolPair{token_a:load6 as u32, token_b:load7 as u32, commission:load8 as u32, reserve_a:load9, reserve_b:load10, total_liquidity:load11, block_commission_a:load12, block_commission_b:load13, }, slop_swap_result:load14, }),
            1 => Err(match load6 {
                0 => Error::RuntimeError,
                1 => Error::NotFoundError,
                2 => Error::InvalidInput,
                3 => Error::LackOfLiquidity,
                4 => Error::PriceHigherThanIndex,
                5 => Error::PoolReserveOverflow,
                _ => return Err(invalid_variant("Error")),
            }),
            _ => return Err(invalid_variant("Result")),
        })
    }
}
use wit_bindgen_wasmtime::rt::RawMem;
use wit_bindgen_wasmtime::rt::invalid_variant;
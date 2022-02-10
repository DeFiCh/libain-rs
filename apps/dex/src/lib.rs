use std::ops::{Add, Sub};
use primitive_types::U256;

use crate::dex::{Error, PoolPair, PoolPrice, SwapResult, TokenAmount};

const COIN: i64 = 100000000;
const MINIMUM_LIQUIDITY: i64 = 1000;
const SLOPE_SWAP_RATE: i64 = 1000;
const PRECISION: u32 = COIN as u32;

mod dex {
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
    #[export_name = "swap"]
    unsafe extern "C" fn __wit_bindgen_swap(arg0: i32, arg1: i32, arg2: i32, arg3: i64, arg4: i64, arg5: i64, arg6: i64, arg7: i64, arg8: i32, arg9: i64, arg10: i64, arg11: i64, arg12: i32, ) -> i32{
        let result0 = <super::Dex as Dex>::swap(PoolPair{token_a:arg0 as u32, token_b:arg1 as u32, commission:arg2 as u32, reserve_a:arg3, reserve_b:arg4, total_liquidity:arg5, block_commission_a:arg6, block_commission_b:arg7, }, TokenAmount{token_id:arg8 as u32, amount:arg9, }, PoolPrice{integer:arg10, fraction:arg11, }, match arg12 {
            0 => false,
            1 => true,
            _ => panic!("invalid enum discriminant"),
        });
        let (result3_0,result3_1,result3_2,result3_3,result3_4,result3_5,result3_6,result3_7,result3_8,result3_9,) = match result0{
            Ok(e) => { {
                let SwapResult{ pool_pair:pool_pair1, slop_swap_result:slop_swap_result1, } = e;
                let PoolPair{ token_a:token_a2, token_b:token_b2, commission:commission2, reserve_a:reserve_a2, reserve_b:reserve_b2, total_liquidity:total_liquidity2, block_commission_a:block_commission_a2, block_commission_b:block_commission_b2, } = pool_pair1;

                (0i32, wit_bindgen_rust::rt::as_i32(token_a2), wit_bindgen_rust::rt::as_i32(token_b2), wit_bindgen_rust::rt::as_i32(commission2), wit_bindgen_rust::rt::as_i64(reserve_a2), wit_bindgen_rust::rt::as_i64(reserve_b2), wit_bindgen_rust::rt::as_i64(total_liquidity2), wit_bindgen_rust::rt::as_i64(block_commission_a2), wit_bindgen_rust::rt::as_i64(block_commission_b2), wit_bindgen_rust::rt::as_i64(slop_swap_result1))
            }}
            Err(e) => { (1i32, e as i32, 0i32, 0i32, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64)}
        };
        let ptr4 = RET_AREA.as_mut_ptr() as i32;
        *((ptr4 + 72) as *mut i64) = result3_9;
        *((ptr4 + 64) as *mut i64) = result3_8;
        *((ptr4 + 56) as *mut i64) = result3_7;
        *((ptr4 + 48) as *mut i64) = result3_6;
        *((ptr4 + 40) as *mut i64) = result3_5;
        *((ptr4 + 32) as *mut i64) = result3_4;
        *((ptr4 + 24) as *mut i32) = result3_3;
        *((ptr4 + 16) as *mut i32) = result3_2;
        *((ptr4 + 8) as *mut i32) = result3_1;
        *((ptr4 + 0) as *mut i32) = result3_0;
        ptr4
    }
    pub trait Dex {
        fn swap(poolpair: PoolPair,token_in: TokenAmount,max_price: PoolPrice,post_bayfront_gardens: bool,) -> Result<SwapResult,Error>;
    }
    static mut RET_AREA: [i64; 10] = [0; 10];
}



impl Add for TokenAmount {
    type Output = TokenAmount;

    fn add(self, rhs: Self) -> Self::Output {
        TokenAmount {
            token_id: self.token_id,
            amount: self.amount.saturating_add(rhs.amount)
        }
    }
}

impl Sub for TokenAmount {
    type Output = TokenAmount;

    fn sub(self, rhs: Self) -> Self::Output {
        TokenAmount {
            token_id: self.token_id,
            amount: self.amount.saturating_sub(rhs.amount)
        }
    }
}

impl PartialEq for TokenAmount {
    fn eq(&self, other: &Self) -> bool {
        self.token_id.eq(&other.token_id)
    }
}


struct Dex {}

impl dex::Dex for Dex {
    fn swap(poolpair: PoolPair, token_in: TokenAmount, max_price: PoolPrice, post_bayfront_gardens: bool) -> Result<SwapResult, Error> {
        let mut poolpair = poolpair;
        if token_in.token_id != poolpair.token_a && token_in.token_id != poolpair.token_b {
            panic!("Error, input token ID ({}) doesn't match pool tokens ({})", poolpair.token_a, poolpair.token_b);
        }
        if token_in.amount <= 0 {
            return Err(Error::InvalidInput);
        }

        let forward = token_in.token_id == poolpair.token_a;

        if poolpair.reserve_a < SLOPE_SWAP_RATE || poolpair.reserve_b < SLOPE_SWAP_RATE {
            return Err(Error::LackOfLiquidity);
        }

        let reserve_a = U256::from(poolpair.reserve_a);
        let reserve_b = U256::from(poolpair.reserve_b);

        let max_prince_256 = U256::from(max_price.integer);
        let price_ab = reserve_a * PRECISION / reserve_b;
        let price_ba = reserve_b * PRECISION / reserve_a;

        let cur_price = if forward { price_ab } else { price_ba };

        if cur_price > max_prince_256 {
            return Err(Error::PriceHigherThanIndex);
        }


        let check_res = if forward { poolpair.reserve_a.checked_add(token_in.amount) } else { poolpair.reserve_b.checked_add(token_in.amount) };
        if check_res.is_none() {
            return Err(Error::PoolReserveOverflow);
        }

        let result = if forward { Dex::slop_swap(token_in.amount, &mut poolpair.reserve_a, &mut poolpair.reserve_b, post_bayfront_gardens) } else { Dex::slop_swap(token_in.amount, &mut poolpair.reserve_b, &mut poolpair.reserve_a, post_bayfront_gardens) };

        Ok(SwapResult {
            pool_pair: poolpair,
            slop_swap_result: result,
        })
    }
}


impl Dex {
    pub fn slop_swap(unswapped: i64, pool_from: &mut i64, pool_to: &mut i64, post_bayfront_gardens: bool) -> i64 {
        let mut unswapped = unswapped;

        assert!(unswapped >= 0);
        assert!(unswapped.checked_add(*pool_from).is_some());

        let mut pool_f = U256::from(*pool_from);
        let mut pool_t = U256::from(*pool_to);

        let mut swapped: U256 = 0.into();

        if post_bayfront_gardens {
            let chunk = if *pool_from / SLOPE_SWAP_RATE < unswapped { *pool_from / SLOPE_SWAP_RATE } else { unswapped };
            while unswapped > 0 {
                let step_from = (chunk).min(unswapped);
                let step_from_256 = U256::from(step_from);
                let step_to = pool_t * step_from_256 / pool_f;
                pool_f += step_from_256;
                pool_t -= step_to;

                unswapped -= step_from;
                swapped += step_to;
            }
        } else {
            let unswapped_a = U256::from(unswapped);
            swapped = pool_t - (pool_t * pool_f / (pool_f * unswapped));
            pool_f += unswapped_a;
            pool_t -= swapped;
        }
        *pool_from = pool_f.as_u64() as i64;
        *pool_to = pool_t.as_u64() as i64;
        return swapped.as_u64() as i64;
    }
}
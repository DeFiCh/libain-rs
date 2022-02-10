use crate::dex::{PoolPrice, TokenAmount};

mod dex {
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
    #[export_name = "swap"]
    unsafe extern "C" fn __wit_bindgen_swap(arg0: i32, arg1: i64, arg2: i64, arg3: i64, arg4: i64, ) -> i64{
        let result0 = <super::Dex as Dex>::swap(TokenAmount{token_id:arg0 as u32, amount:arg1, }, arg2, PoolPrice{integer:arg3, fraction:arg4, });
        wit_bindgen_rust::rt::as_i64(result0)
    }
    pub trait Dex {
        fn swap(in_: TokenAmount,dexfee_in_pct: i64,max_price: PoolPrice,) -> i64;
    }
}


struct Dex{}

impl dex::Dex for Dex {
    fn swap(in_: TokenAmount, dexfee_in_pct: i64, max_price: PoolPrice) -> i64 {
        // TODO perform dex swap here
        todo!()
    }
}

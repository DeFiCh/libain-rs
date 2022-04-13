use cpp::{cpp, cpp_class};
use crate::types::*;

cpp!{{
    #include <masternodes/accounts.h>

}}

cpp_class!(pub unsafe struct AccountStorage as "CAccountsView");


impl AccountStorage {
    pub fn get_balance(owner : Script, token_id : DctId) -> TokenAmount {
        unsafe {
            cpp!([owner as "CScript", token_id as "DCT_ID"] -> TokenAmount as "CTokenAmount" {
                return CAccountsView().GetBalance(owner, token_id);
            })
        }
    }

    pub fn add_balance(owner : Script, amount : TokenAmount) -> Res {
        unsafe {
            cpp!([ owner as "CScript", amount as "CTokenAmount"] -> Res as "Res" {
                return CAccountsView().AddBalance(owner, amount);
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    cpp!{{
        #include <test/setup_common.h>
        #include <masternodes/masternodes.h>
         #include <flushablestorage.h>
    }}



    #[test]
    fn test_account_storage() {
        cpp!({
            auto pcustomcsDB = std::make_shared<CStorageKV>(CStorageLevelDB("level_db_account_storage", 3000000, true, false));
            auto storage = CFlushableStorageKV(pcustomcsDB);
        });

        let res = AccountStorage::add_balance(Script::from_vec(vec![0]), TokenAmount::new(DctId::new(30), 1000));
    }
}

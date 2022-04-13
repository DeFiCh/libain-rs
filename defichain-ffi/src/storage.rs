use cpp::{cpp, cpp_class};
use crate::types::*;

cpp! {{
    #include <masternodes/accounts.h>
#include <masternodes/accounts.h>
}}

cpp_class!(pub unsafe struct AccountStorage as "CAccountsView");
cpp_class!(pub unsafe struct DB as "std::shared_ptr<CStorageKV>");


impl AccountStorage {
    pub fn get_balance(owner: Script, token_id: DctId) -> TokenAmount {
        unsafe {
            cpp!([owner as "CScript", token_id as "DCT_ID"] -> TokenAmount as "CTokenAmount" {
                return CAccountsView().GetBalance(owner, token_id);
            })
        }
    }

    pub fn add_balance(owner: Script, amount: TokenAmount) -> Res {
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


    #[test]
    fn test_account_storage() {
        // unsafe { cpp!([self as "const MyClass*", param as "int"] -> i32 as "int" {
        ///             return self->member_function(param);
        ///         }) }
        unsafe {
            cpp!([] {
                auto pcustomcsDB = std::make_shared<CStorageKV>(CStorageLevelDB("level_db_account_storage", 3000000, true, false));
                CStorageView sview = CStorageView(pcustomcsDB);
                CScript const owner = CScript(1);
                auto dfi100 = CTokenAmount{DCT_ID{0}, 100};
                sview.WriteBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, dfi100.nTokenId}, dfi100.nValue);
            }
        )
        }

        //let res = AccountStorage::add_balance(Script::new(1), TokenAmount::new(DctId::new(0), 1000));
    }
}

use cpp::{cpp, cpp_class};
use crate::types::*;
use crate::view::CustomStorageView;

cpp! {{
    #include <masternodes/accounts.h>
    #include <flushablestorage.h>
}}

pub trait AccountsView {
    fn set_balance(&self, owner: Script, amount: TokenAmount);
    fn get_balance(&self, owner: Script, token_id: DctId) -> TokenAmount;
}


impl AccountsView for CustomStorageView {
    fn set_balance(&self, owner: Script, amount: TokenAmount) {
        unsafe {
            let db = &self.inner;
            cpp!([ db as "std::unique_ptr<CStorageView>*", owner as "CScript", amount as "CTokenAmount"] {
                (*db)->WriteBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, amount.nTokenId}, amount.nValue);
            })
        };
    }

    fn get_balance(&self, owner: Script, token_id: DctId) -> TokenAmount {
        unsafe {
            let db = &self.inner;
            cpp!([ db as "std::unique_ptr<CStorageView>*", owner as "CScript", token_id as "DCT_ID"] -> TokenAmount as "CTokenAmount" {
                CAmount val;
                bool ok = (*db)->ReadBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, token_id}, val);
                if (ok) {
                    return CTokenAmount{token_id, val};
                }
                return CTokenAmount{token_id, 0};
            })
        }
    }
}


#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use crate::view::RawStorageKVPtr;
    use super::*;

    fn init_test_storage(dir: &str) -> RawStorageKVPtr {
        unsafe {
            let path = dir.to_string();
            cpp!([path as "rust::String"] -> RawStorageKVPtr as "std::shared_ptr<CStorageKV>"{
                std::string std_path = std::string(path);
                auto pcustomcsDB = std::make_shared<CStorageKV>(CStorageLevelDB(std_path, 3000000, false, false));
                return pcustomcsDB;
            }
        )
        }
    }


    #[test]
    fn test_account_storage() {
        let tmp_dir = TempDir::new("__test__test_account_storage__").expect("unable to create test directory");
        let db = init_test_storage(tmp_dir.path().to_str().unwrap());
        let mnview = CustomStorageView::new(db);
        let script = Script::from_vec(vec![1, 2, 3, 4]);
        mnview.set_balance(script.clone(), TokenAmount::new(DctId::new(0), 10000));
        mnview.flush();
        assert_eq!(mnview.get_balance(script, DctId::new(0)), TokenAmount::new(DctId::new(0), 10000))
    }
}

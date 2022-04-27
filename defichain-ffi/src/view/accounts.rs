use cpp::{cpp, cpp_class};
use crate::types::*;

cpp! {{
    #include <masternodes/accounts.h>
    #include <flushablestorage.h>
    #include <iostream>
    #include <stdio.h>
}}

cpp_class!(pub unsafe struct CustomDB as "std::shared_ptr<CStorageKV>");

pub struct Accounts {
    db: CustomDB,
}


impl Accounts {
    pub fn set_balance(&self, owner: Script, amount: TokenAmount) {
        unsafe {
            let db = self.db.clone();
            cpp!([ db as "std::shared_ptr<CStorageKV>", owner as "CScript", amount as "CTokenAmount"] {
                CStorageView sview = CStorageView(db);
                sview.WriteBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, amount.nTokenId}, amount.nValue);
            })
        };
    }

    pub fn get_balance(&self, owner: Script, token_id: DctId) -> TokenAmount {
        unsafe {
            let db = self.db.clone();
            cpp!([ db as "std::shared_ptr<CStorageKV>", owner as "CScript", token_id as "DCT_ID"] -> TokenAmount as "CTokenAmount" {
                CAmount val;
                CStorageView sview = CStorageView(db);
                bool ok = sview.ReadBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, token_id}, val);
                if (ok) {
                    return CTokenAmount{token_id, val};
                }
                return CTokenAmount{token_id, 0};
            })
        }
    }
}

impl CustomDB {
    pub fn flush(self)  {
        unsafe {
            cpp!([self as "std::shared_ptr<CStorageKV>"]{
                CStorageView sview = CStorageView(self);
                sview.Flush();
            })
        }
    }
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use super::*;

    fn init_test_storage(dir: &str) -> CustomDB {
        unsafe {
            let path = dir.to_string();
            cpp!([path as "rust::String"] -> CustomDB as "std::shared_ptr<CStorageKV>"{
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
        let accounts = Accounts {
            db : db.clone()
        };
        let script = Script::from_vec(vec![1, 2, 3, 4]);
        accounts.set_balance(script.clone(), TokenAmount::new(DctId::new(0), 10000));
        db.flush();
        assert_eq!(accounts.get_balance(script, DctId::new(0)), TokenAmount::new(DctId::new(0), 10000))
    }
}

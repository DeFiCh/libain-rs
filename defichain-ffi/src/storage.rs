use cpp::{cpp, cpp_class};
use crate::types::*;

cpp! {{
    #include <masternodes/accounts.h>
    #include <masternodes/masternodes.h>
    #include <flushablestorage.h>
    #include <iostream>
    #include <stdio.h>
}}

cpp_class!(pub unsafe struct CustomDB as "std::shared_ptr<CStorageKV>");
//cpp_class!(pub unsafe struct DB as "std::shared_ptr<CStorageKV>");

pub struct  Accounts {
    db : CustomDB
}


impl Accounts {

    pub fn set_balance(&self, owner: Script, amount: TokenAmount)  {
        unsafe {
            let db = self.db.clone();
            //let b = owner;
            cpp!([ db as "std::shared_ptr<CStorageKV>", owner as "CScript", amount as "CTokenAmount"] {
                CStorageView sview = CStorageView(db);
                sview.WriteBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, amount.nTokenId}, amount.nValue);
                sview.Flush();
                //sview.Write(uint64_t(1), amount.nValue);
            })
        };
    }

    pub fn get_balance(&self, owner: Script, token_id: DctId) -> TokenAmount {
        unsafe {
            let db = self.db.clone();
            //let b = owner;
            cpp!([ db as "std::shared_ptr<CStorageKV>", owner as "CScript", token_id as "DCT_ID"] -> TokenAmount as "CTokenAmount" {
                CAmount val;
                CStorageView sview = CStorageView(db);

                bool ok = sview.ReadBy<CAccountsView::ByBalanceKey>(BalanceKey{owner, token_id}, val);
                //bool ok = sview.Read(uint64_t(1), val);
                if (ok) {
                    printf("OK");
                    return CTokenAmount{token_id, val};
                }
                return CTokenAmount{token_id, 0};
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_account_storage() {
        let db = unsafe {
            cpp!([] -> CustomDB as "std::shared_ptr<CStorageKV>"{
                auto pcustomcsDB = std::make_shared<CStorageKV>(CStorageLevelDB("level_db_account_storage", 3000000, false, false));
                return pcustomcsDB;
            }
        )
        };

        let accounts = Accounts {
            db
        };
        let script = Script::from_vec(vec![1,2,3,4]);
        println!("{}", script.get_hex());
        accounts.set_balance(script.clone(), TokenAmount::new(DctId::new(0), 10000));
        println!("{:#?}", accounts.get_balance(script, DctId::new(0)))
    }
}

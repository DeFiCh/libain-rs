use std::error::Error;
use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use cpp::{cpp, cpp_class};
use cxx::{CxxString, CxxVector, O};
use std::os::raw::c_char;
use crate::error::FFIError;
cpp! {{
    #include <amount.h>
    #include "cxx.h"
    #include <script/script.h>
    #include <masternodes/res.h>
}}

cpp_class! {
     #[derive(PartialEq, PartialOrd)]
    pub unsafe struct DctId as "DCT_ID"
}
cpp_class!(pub unsafe struct Script as "CScript");
cpp_class! {
    #[derive(PartialEq)]
    pub unsafe struct TokenAmount as "CTokenAmount"
}

cpp_class!(pub unsafe struct Res as "Res");

impl DctId {
    pub fn new(v: u32) -> Self {
        unsafe {
            cpp! { [v as "uint32_t"] -> DctId as "DCT_ID" {
                    return DCT_ID{v};
                }
            }
        }
    }

    pub fn value(&self) -> u32 {
        unsafe {
            cpp! { [self as "const DCT_ID*"] -> u32 as "uint32_t" {
                    return self->v;
                }
            }
        }
    }
}

impl Debug for DctId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.value())
    }
}

impl Script {
    pub fn from_vec(v: Vec<u8>) -> Self {
        unsafe {
            cpp! { [v as "rust::Vec<unsigned char>"] -> Script as "CScript" {
                    std::vector<unsigned char> stdv;
                    std::copy(v.begin(), v.end(), std::back_inserter(stdv));
                    return CScript(stdv.begin(), stdv.end());
                }
            }
        }
    }

    pub fn get_hex(&self) -> String {
        unsafe {
            cpp!([self as "const CScript*"] -> String as "rust::String" {
            return self->GetHex();
        })
        }
    }

    pub fn has_valid_ops(&self) -> bool {
        unsafe {
            cpp!([self as "const CScript*"] -> bool as "bool" {
            return self->HasValidOps();
        })
        }
    }


    pub fn is_pay_to_script_hash(&self) -> bool {
        unsafe {
            cpp!([self as "const CScript*"] -> bool as "bool" {
            return self->IsPayToScriptHash();
        })
        }
    }

    pub fn is_pay_to_witness_script_hash(&self) -> bool {
        unsafe {
            cpp!([self as "const CScript*"] -> bool as "bool" {
            return self->IsPayToWitnessScriptHash();
        })
        }
    }
}

impl TokenAmount {
    pub fn new(token_id: DctId, amount: u64) -> Self {
        unsafe {
            cpp! { [token_id as "DCT_ID", amount as "int64_t"] -> TokenAmount as "CTokenAmount" {
                    return CTokenAmount{token_id,amount};
                }
            }
        }
    }

    pub fn token_id(&self) -> DctId {
        unsafe {
            cpp!([self as "const CTokenAmount*"] -> DctId as "DCT_ID" {
            return self->nTokenId;
        })
        }
    }

    pub fn amount(&self) -> u64 {
        unsafe {
            cpp!([self as "const CTokenAmount*"] -> u64 as "int64_t" {
            return self->nValue;
        })
        }
    }
}

impl Res {
    fn msg(&self) -> String {
        unsafe {
            cpp!([self as "const Res*"] -> String as "rust::String" {
            return self->msg;
        })
        }
    }

    fn code(&self) -> u32 {
        unsafe {
            cpp!([self as "const Res*"] -> u32 as "uint32_t" {
            return self->code;
        })
        }
    }

    fn debug_msg(&self) -> String {
        unsafe {
            cpp!([self as "const Res*"] -> String as "rust::String" {
            return self->dbgMsg;
        })
        }
    }

    fn ok(&self) -> bool {
        unsafe {
            cpp!([self as "const Res*"] -> bool as "bool" {
            return self->ok;
        })
        }
    }

    fn into_result(self) -> Result<String, FFIError> {
        return if self.ok() {
            Ok(self.msg())
        } else {
            Err(FFIError::DefiChainError(self.debug_msg()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dct_id_type() {
        let dct = DctId::new(10);
        assert_eq!(dct.value(), 10_u32)
    }

    #[test]
    fn test_token_amount_type() {
        let dct = DctId::new(10);
        println!("{:?}", dct);
        let token_amount = TokenAmount::new(dct, 1000);
        assert_eq!(token_amount.token_id(), dct);
        assert_eq!(token_amount.amount(), 1000);
    }

    #[test]
    fn test_script_type() {
        let encoded_hex = "76a9141234567890abcdefa1a2a3a4a5a6a7a8a9a0aaab88ac";
        let decoded_hex = hex::decode(encoded_hex).unwrap();
        let script = Script::from_vec(decoded_hex);
        assert_eq!(script.get_hex(), encoded_hex);
        assert!(script.has_valid_ops())
    }
}


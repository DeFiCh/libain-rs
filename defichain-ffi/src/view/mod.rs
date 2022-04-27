use cpp::*;

mod accounts;

cpp! {{
    #include <masternodes/accounts.h>
    #include <flushablestorage.h>
}}


cpp_class!(pub unsafe struct RawStorageKVPtr as "std::shared_ptr<CStorageKV>");
cpp_class!(pub unsafe struct StorageViewPtr as "std::unique_ptr<CStorageView>");

pub struct CustomStorageView {
    inner : StorageViewPtr
}

/// State
impl CustomStorageView {
    pub fn new(kv: RawStorageKVPtr) -> Self {
        let db = cpp!(unsafe [kv as "std::shared_ptr<CStorageKV>"] -> StorageViewPtr as "std::unique_ptr<CStorageView>"{
            std::unique_ptr<CStorageView> storage_view = std::make_unique<CStorageView>(kv);
            return storage_view;
        });

        Self {
            inner: db
        }
    }


    pub fn flush(&self) {
        let db = &self.inner;
        cpp!(unsafe [ db as "std::unique_ptr<CStorageView>*"] {
            (*db)->Flush();
        });
    }
}

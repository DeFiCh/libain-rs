// use serde::Serialize;

pub mod types {
    tonic::include_proto!("types");
}

pub mod rpc {
    tonic::include_proto!("rpc");
}

// impl Serialize for union::BlockResult {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match &self.result {
//             Some(union::block_result::Result::HexData(ref data)) => serializer.serialize_str(data),
//             Some(union::block_result::Result::Block(ref block)) => block.serialize(serializer),
//             None => serializer.serialize_str(""),
//         }
//     }
// }

// impl Serialize for union::Transaction {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match &self.info {
//             Some(union::transaction::Info::Hash(ref data)) => serializer.serialize_str(data),
//             Some(union::transaction::Info::Raw(ref transaction)) => {
//                 transaction.serialize(serializer)
//             }
//             None => serializer.serialize_str(""),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::union::{Transaction, transaction, BlockResult, block_result};

//     #[test]
//     fn test_block_result_json() {
//         let foo = BlockResult {
//             result: Some(block_result::Result::HexData("foo".into())),
//         };
//         assert_eq!(serde_json::to_value(&foo).unwrap(), "foo");
//     }

//     #[test]
//     fn test_transaction() {
//         let foo = Transaction {
//             info: Some(transaction::Info::Hash("foo".into())),
//         };
//         assert_eq!(serde_json::to_value(&foo).unwrap(), "foo");
//     }
// }

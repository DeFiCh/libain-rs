use serde::Serialize;

pub mod types {
    tonic::include_proto!("types");
}

pub mod rpc {
    tonic::include_proto!("rpc");
}

impl Serialize for types::BlockResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.hash != "" {
            return serializer.serialize_str(&self.hash);
        }

        if let Some(ref block) = self.block {
            return block.serialize(serializer);
        }

        serializer.serialize_str("")
    }
}

impl Serialize for types::Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.hash != "" {
            return serializer.serialize_str(&self.hash);
        }

        if let Some(ref tx) = self.raw {
            return tx.serialize(serializer);
        }

        serializer.serialize_str("")
    }
}

#[cfg(test)]
mod tests {
    use super::types::{BlockResult, Transaction};

    #[test]
    fn test_block_result_json() {
        let foo = BlockResult {
            hash: "foobar".into(),
            block: None,
        };
        assert_eq!(serde_json::to_value(&foo).unwrap(), "foobar");
    }

    #[test]
    fn test_transaction() {
        let foo = Transaction {
            hash: "booya".into(),
            raw: None,
        };
        assert_eq!(serde_json::to_value(&foo).unwrap(), "booya");
    }
}

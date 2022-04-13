use thiserror::Error;

#[derive(Error, Debug)]
pub enum FFIError {
    #[error("`{0}`")]
    DefiChainError(String),
}
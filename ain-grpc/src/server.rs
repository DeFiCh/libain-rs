use tonic::transport::Server;

use crate::codegen::rpc::BlockchainService;

use std::error::Error;

pub async fn serve(addr: &str) -> Result<(), Box<dyn Error>> {
    let addr = addr.parse().expect("invalid address");
    Server::builder()
        .add_service(BlockchainService::service())
        .serve(addr)
        .await?;
    Ok(())
}

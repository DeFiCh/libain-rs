#[macro_use]
extern crate serde;
extern crate serde_json;

mod codegen;

use tonic::transport::Server;

use crate::codegen::rpc::BlockchainService;

use std::error::Error;

struct Runtime {
    inner: tokio::runtime::Runtime,
}

#[cxx::bridge]
mod server {
    extern "Rust" {
        type Runtime;

        fn new_runtime() -> Result<Box<Runtime>>;

        fn start_servers(
            runtime: Box<Runtime>,
            json_addr: &str,
            grpc_addr: &str,
        ) -> Result<Box<Runtime>>;

        fn stop_servers(runtime: Box<Runtime>) -> Result<()>;
    }
}

fn new_runtime() -> Result<Box<Runtime>, Box<dyn Error>> {
    Ok(Box::new(Runtime {
        inner: tokio::runtime::Runtime::new()?,
    }))
}

fn start_servers(
    runtime: Box<Runtime>,
    json_addr: &str,
    grpc_addr: &str,
) -> Result<Box<Runtime>, Box<dyn Error>> {
    (&*runtime).inner.spawn(
        Server::builder()
            .add_service(BlockchainService::service())
            .serve(grpc_addr.parse()?),
    );
    Ok(runtime)
}

fn stop_servers(runtime: Box<Runtime>) -> Result<(), Box<dyn Error>> {
    // Dropping the runtime should block until the resources are freed
    Ok(())
}

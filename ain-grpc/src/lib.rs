#[macro_use]
extern crate serde;
extern crate serde_json;

mod codegen;

use env_logger::{Builder, Env};
use jsonrpsee::http_server::HttpServerBuilder;
use log::Level;
use tonic::transport::Server;

use crate::codegen::rpc::BlockchainService;

use std::error::Error;
use std::net::SocketAddr;

struct Runtime {
    inner: tokio::runtime::Runtime,
}

#[cxx::bridge]
mod server {
    extern "Rust" {
        type Runtime;

        fn init_runtime() -> Result<Box<Runtime>>;

        fn start_servers(
            runtime: Box<Runtime>,
            json_addr: &str,
            grpc_addr: &str,
        ) -> Result<Box<Runtime>>;

        fn stop_servers(runtime: Box<Runtime>) -> Result<()>;
    }
}

fn init_runtime() -> Result<Box<Runtime>, Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or(Level::Info.as_str())).init();
    Ok(Box::new(Runtime {
        inner: tokio::runtime::Runtime::new()?,
    }))
}

fn start_servers(
    runtime: Box<Runtime>,
    json_addr: &str,
    grpc_addr: &str,
) -> Result<Box<Runtime>, Box<dyn Error>> {
    let json_addr = json_addr.parse::<SocketAddr>()?;
    log::info!("Starting JSON RPC server at {}", json_addr);
    let server = runtime.inner.block_on(
        HttpServerBuilder::default()
            .custom_tokio_runtime(runtime.inner.handle().clone())
            .build(json_addr),
    )?;
    server.start(BlockchainService::module()?)?;
    log::info!("Starting gRPC server at {}", grpc_addr);
    (&*runtime).inner.spawn(
        Server::builder()
            .add_service(BlockchainService::service())
            .serve(grpc_addr.parse()?),
    );
    Ok(runtime)
}

fn stop_servers(_runtime: Box<Runtime>) -> Result<(), Box<dyn Error>> {
    log::info!("Stopping gRPC and JSON RPC servers");
    // Dropping the runtime should block until the resources are freed
    Ok(())
}

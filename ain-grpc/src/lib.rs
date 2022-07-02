#[macro_use]
extern crate serde;
extern crate serde_json;

mod codegen;

use env_logger::{Builder as LogBuilder, Env};
use jsonrpsee::http_server::{HttpServerBuilder, HttpServerHandle};
use log::Level;
use tokio::runtime::{Builder, Runtime as AsyncRuntime};
use tokio::sync::mpsc::{self, Sender};
use tonic::transport::Server;

use crate::codegen::rpc::{BlockchainService, MiningService, WalletService};

use std::error::Error;
use std::net::SocketAddr;
use std::thread::{self, JoinHandle};

struct Runtime {
    rt: Option<AsyncRuntime>,
    tx: Option<Sender<()>>,
    handle: Option<JoinHandle<()>>,
    jrpc_handle: Option<HttpServerHandle>, // dropping the handle kills server
}

impl Runtime {
    fn new_boxed() -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(Runtime {
            rt: Some(Builder::new_multi_thread().enable_all().build()?),
            tx: None,
            handle: None,
            jrpc_handle: None,
        }))
    }

    fn add_json_rpc_server(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        log::info!("Starting JSON RPC server at {}", addr);
        let addr = addr.parse::<SocketAddr>()?;
        let rt = self.rt.as_ref().expect("uninitialized runtime");
        let handle = rt.handle().clone();
        let server = rt.block_on(
            HttpServerBuilder::default()
                .custom_tokio_runtime(handle)
                .build(addr),
        )?;
        let mut methods = BlockchainService::module()?;
        methods.merge(WalletService::module()?)?;
        methods.merge(MiningService::module()?)?;
        self.jrpc_handle = Some(server.start(methods)?);
        Ok(())
    }

    fn add_grpc_server(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        log::info!("Starting gRPC server at {}", addr);
        self.rt.as_ref().expect("uninitialized runtime").spawn(
            Server::builder()
                .add_service(BlockchainService::service())
                .add_service(MiningService::service())
                .add_service(WalletService::service())
                .serve(addr.parse()?),
        );
        Ok(())
    }

    fn start(&mut self) {
        let (tx, mut rx) = mpsc::channel(1);
        self.tx = Some(tx);
        let rt = self.rt.take().expect("uninitialized runtime");
        self.handle = Some(thread::spawn(move || {
            log::info!("Starting runtime in a separate thread");
            rt.block_on(async move {
                rx.recv().await;
            });
        }));
    }

    fn stop(&mut self) {
        let _ = self
            .tx
            .take()
            .expect("uninitialized runtime")
            .blocking_send(());
        self.handle.take().unwrap().join().unwrap();
    }
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
    LogBuilder::from_env(Env::default().default_filter_or(Level::Info.as_str())).init();
    Runtime::new_boxed()
}

fn start_servers(
    mut runtime: Box<Runtime>,
    json_addr: &str,
    grpc_addr: &str,
) -> Result<Box<Runtime>, Box<dyn Error>> {
    runtime.add_json_rpc_server(json_addr)?;
    runtime.add_grpc_server(grpc_addr)?;
    runtime.start();
    Ok(runtime)
}

fn stop_servers(mut runtime: Box<Runtime>) -> Result<(), Box<dyn Error>> {
    log::info!("Stopping gRPC and JSON RPC servers");
    runtime.stop();
    Ok(())
}

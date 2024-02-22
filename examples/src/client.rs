use krpc_core::{client::KrpcClient, register::{RegisterBuilder, RegisterType}};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tracing::info;
use krpc_common::RpcError;
use krpc_macro::rpc_resources;

lazy_static! {
    static ref CLI: KrpcClient = KrpcClient::build(
        RegisterBuilder::new(
            &format!("127.0.0.1:{}", "2181"),
            "default",
            RegisterType::ZooKeeper,
        )
    );
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct ReqDto {
    str: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct ResDto {
    str: String,
}

#[rpc_resources(package = "com.krpc", version = "1.0.0")]
trait TestServer {
    async fn do_run1(&self, res1: ReqDto, res2: ResDto) -> ResDto;
    async fn do_run2(&self, res: ReqDto) -> ResDto;
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    let de = TestServer { client: &CLI };
    krpc_common::init_log();
    let client = de;
    let res = client.do_run1(
        ReqDto { str: "client say hello 1".to_string() },
        ResDto { str: "client say hello 2".to_string() }).await;
    info!("{:?}",res);
    let res = client.do_run2(ReqDto { str: "client say hello 2".to_string() }).await;
    info!("{:?}",res);
}



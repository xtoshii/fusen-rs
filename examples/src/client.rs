use std::sync::Arc;

use examples::{DemoServiceClient, ReqDto};
use fusen_rs::fusen_common::register::Type;
use fusen_rs::fusen_common::url::UrlConfig;
use fusen_rs::fusen_macro::handler;
use fusen_rs::handler::loadbalance::LoadBalance;
use fusen_rs::handler::HandlerLoad;
use fusen_rs::protocol::socket::InvokerAssets;
use fusen_rs::register::nacos::NacosConfig;
use fusen_rs::{fusen_common, FusenApplicationContext};
use tracing::info;

struct CustomLoadBalance;

#[handler(id = "CustomLoadBalance")]
impl LoadBalance for CustomLoadBalance {
    async fn select(
        &self,
        _invokers: Vec<Arc<InvokerAssets>>,
    ) -> Result<Arc<InvokerAssets>, fusen_rs::Error> {
        todo!()
    }
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    fusen_common::logs::init_log();
    let context = FusenApplicationContext::builder()
        .add_register_builder(
            NacosConfig::builder()
                .server_addr("127.0.0.1:8848".to_owned())
                .app_name(Some("fusen-service".to_owned()))
                .server_type(Type::Fusen)
                .build()
                .boxed(),
        )
        .add_register_builder(
            NacosConfig::builder()
                .server_addr("127.0.0.1:8848".to_owned())
                .app_name(Some("service-provider".to_owned()))
                .server_type(Type::SpringCloud)
                .build()
                .boxed(),
        )
        .add_register_builder(
            NacosConfig::builder()
                .server_addr("127.0.0.1:8848".to_owned())
                .app_name(Some("dubbo-client".to_owned()))
                .server_type(Type::Dubbo)
                .build()
                .boxed(),
        )
        .add_handler(CustomLoadBalance.load())
        .build();
    //进行Fusen协议调用HTTP2 + JSON
    let client = DemoServiceClient::new(context.client(Type::Fusen).unwrap());
    let res = client
        .sayHelloV2(ReqDto {
            str: "world".to_string(),
        })
        .await;
    info!("rev fusen msg : {:?}", res);

    //进行Dubbo3协议调用HTTP2 + GRPC
    let client = DemoServiceClient::new(context.client(Type::Dubbo).unwrap());
    let res = client.sayHello("world".to_string()).await;
    info!("rev dubbo3 msg : {:?}", res);

    //进行SpringCloud协议调用HTTP1 + JSON
    let client = DemoServiceClient::new(context.client(Type::SpringCloud).unwrap());
    let res = client.divideV2(1, 2).await;
    info!("rev springcloud msg : {:?}", res);
}

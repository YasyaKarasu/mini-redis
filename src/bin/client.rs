use lazy_static::lazy_static;
use std::net::SocketAddr;

lazy_static! {
    static ref CLIENT: volo_gen::mini::redis::MiniRedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini::redis::MiniRedisServiceClientBuilder::new("volo-example")
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let resp = CLIENT.ping().await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
    let req = volo_gen::mini::redis::SetValueRequest{
        key: "foo".into(),
        value: "bar".into(),
        expire_seconds: Some(5),
    };
    let resp = CLIENT.set_value(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
    let req = volo_gen::mini::redis::GetValueRequest{
        key: "foo".into(),
    };
    let resp = CLIENT.get_value(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
    let req = volo_gen::mini::redis::DeleteValueRequest{
        key: "foo".into(),
    };
    let resp = CLIENT.delete_value(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
    let req = volo_gen::mini::redis::GetValueRequest{
        key: "foo".into(),
    };
    let resp = CLIENT.get_value(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
}

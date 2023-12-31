#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use mini_redis::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::mini::redis::MiniRedisServiceServer::new(S{
        map: std::sync::Mutex::new(std::collections::HashMap::new())
    })
        .run(addr)
        .await
        .unwrap();
}

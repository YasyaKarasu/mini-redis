use std::env;
use std::process;
use lazy_static::lazy_static;
use std::net::SocketAddr;

lazy_static! {
    static ref CLIENT: volo_gen::mini::redis::MiniRedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini::redis::MiniRedisServiceClientBuilder::new("mini-redis")
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        eprintln!("Usage: redis_cli [OPTIONS] [command] [arg]");
        eprintln!("Options:");
        eprintln!("    -h --help                                     Print this help message.");
        eprintln!("Commands:");
        eprintln!("    ping                                          Ping the server.");
        eprintln!("    set [key] [val] ([expire_seconds])            Set the value of a key.");
        eprintln!("    get [key]                                     Get the value of a key.");
        eprintln!("    del [key]                                     Delete a key-value pair.");
        process::exit(0);
    }
    args[1].make_ascii_lowercase();
    match args[1] {
        ref cmd if cmd == "ping" => {
            let resp = CLIENT.ping().await;
            match resp {
                Ok(info) => tracing::info!("{:?}", info),
                Err(e) => tracing::error!("{:?}", e),
            }
        },
        ref cmd if cmd == "set" => {
            if args.len() != 4 && args.len() != 5{
                eprintln!("Usage: redis_cli set [key] [value] ([expire_seconds])");
                process::exit(1);
            }
            let mut expire_seconds = 0;
            if args.len() == 5 {
                expire_seconds = args[4].parse().unwrap();
            }
            let req = volo_gen::mini::redis::SetValueRequest{
                key: args[2].clone().into(),
                value: args[3].clone().into(),
                expire_seconds: Some(expire_seconds),
            };
            let resp = CLIENT.set_value(req).await;
            match resp {
                Ok(_) => println!("OK"),
                Err(e) => tracing::error!("{:?}", e),
            }
        },
        ref cmd if cmd == "get" => {
            if args.len() != 3 {
                eprintln!("Usage: redis_cli get [key]");
                process::exit(1);
            }
            let req = volo_gen::mini::redis::GetValueRequest{
                key: args[2].clone().into(),
            };
            let resp = CLIENT.get_value(req).await;
            match resp {
                Ok(info) => println!("{}", info.value),
                Err(e) => tracing::error!("{:?}", e),
            }
        },
        ref cmd if cmd == "del" => {
            if args.len() != 3 {
                eprintln!("Usage: redis_cli del [key]");
                process::exit(1);
            }
            let req = volo_gen::mini::redis::DeleteValueRequest{
                key: args[2].clone().into(),
            };
            let resp = CLIENT.delete_value(req).await;
            match resp {
                Ok(_) => println!("OK"),
                Err(e) => tracing::error!("{:?}", e),
            }
        },
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
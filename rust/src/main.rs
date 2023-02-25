use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1)
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    // let address: &str = &args[3];
    match protocol {

        "tcp" => match role {
            "server" => {
                // TODO: TCPサーバー呼び出し
            }
            "client" => {
            }
            _ => {
                error!("Please specify server or client on the 2nd argument.");
                std::process::exit(1);
            }
        },
        "upd" => match role {
            "server" => {}
            "client" => {}
            _ => {
                error!("Please specify server or client on the 2nd argument.");
                std::process::exit(1);
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}



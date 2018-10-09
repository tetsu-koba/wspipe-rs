extern crate wspipe;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let socaddr = if args.len() < 2 {
        "0.0.0.0:8001"
    } else {
        &args[1]
    };
    wspipe::websocket2stdout(socaddr);
}

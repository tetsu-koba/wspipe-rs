extern crate wspipe;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = if args.len() < 2 {
        "ws://127.0.0.1:8001"
    } else {
        &args[1]
    };
    wspipe::stdin2websocket(url);
}

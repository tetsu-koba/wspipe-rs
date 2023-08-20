extern crate ws;

use ws::{connect, CloseCode};
use std::{thread, time};
use std::sync::mpsc::channel;
use std::io::{Read, Write};

fn send_binary(out: &ws::Sender) {
    let stdin = std::io::stdin();
    let mut infile = stdin.lock();
    let mut buf = [0u8; 8 * 1024];
    loop {
        match infile.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if let Err(e) = out.send(&buf[0..n]) {
                    eprintln!("websocket send error: {:?}", e);
                    break;
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }
}

fn msg_handler(_msg: ws::Message) -> ws::Result<()> {
    //print!("Client got message '{}'. ", msg);
    Ok(())
}

pub fn stdin2websocket(url: &str) {
    let (sender, receiver) = channel();
    if let Err(error) = connect(url, |out| {
        let s2 = sender.clone();
        thread::spawn(move || {
            send_binary(&out);
            thread::sleep(time::Duration::from_millis(100));
            out.close(CloseCode::Normal).unwrap();
            s2.send(()).unwrap();
        });

        msg_handler
    })
    {
        println!("Failed to create WebSocket due to: {:?}", error);
    }
    receiver.recv().unwrap();
}

pub fn websocket2stdout(socaddr: &str) {
    if let Err(error) = ws::listen(socaddr, |_wsout| {
        move |msg| {
            let stdout = std::io::stdout();
            let mut out = stdout.lock();
            use ws::Message::{Text, Binary};
            match msg {
                Text(str) => write!(out, "{}", str),
                Binary(d) => out.write_all(&d),
            }?;
            out.flush()?;
            Ok(())
        }
    })
    {
        eprintln!("Failed to create WebSocket due to {:?}", error);
    }
}

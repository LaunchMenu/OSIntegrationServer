//https://github.com/microsoft/windows-rs
use std::io;
use std::sync::mpsc::*;
use std::thread;

// use std::io::{self, Read};

struct Message {
    uid: i64,
    data: json::JsonValue,
}

static mut ID: i64 = 0;
fn counter() -> i64 {
    unsafe {
        ID = ID + 1;
        return ID;
    }
}

fn main() {
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();
    thread::spawn(move || loop {
        let m = receiver.recv();
        match m {
            Ok(msg) => {
                //TODO: implement feature lookup and running here.

                println!("{},{}", msg.uid, msg.data);
            }
            Err(_) => {
                println!("Some error");
            }
        }
    });

    loop {
        //Get string from stdin
        let mut buffer = String::new();
        let stdin = io::stdin();
        let _ = stdin.read_line(&mut buffer);
        let data = json::parse(&buffer).unwrap();
        //
        sender
            .send(Message {
                uid: counter(),
                data,
            })
            .unwrap();
    }
}

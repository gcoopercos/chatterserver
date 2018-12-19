extern crate appnetcore;

use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use appnetcore::reader::CommCommand;
use appnetcore::reader::AppCommand;
use appnetcore::reader::PacketReaderServer;
use appnetcore::reader::{check_app_commands, check_comm_commands};
use appnetcore::network::read_packets;
use appnetcore::connstate::SocketReadAddress;

use std::time::{SystemTime, UNIX_EPOCH};

//const  THRESHOLD: i32 = 10;
const MS_PER_UPDATE: f64 = 60.0;

fn get_current_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    in_ms
}

fn main() {
    println!("Initialization...");
    let listen_address = SocketReadAddress{
        read_host: String::from("127.0.0.1"),
        read_port: 10000,
        reader_id: String::from("testserver")
    };

    // States.
    let mut client_state: HashMap<String,SocketReadAddress> = HashMap::new();

    let (tx,command_rx): (Sender<Box<CommCommand + Send>>,
                          Receiver<Box<CommCommand + Send>>) = mpsc::channel();

    let (app_tx,app_command_rx): (Sender<Box<AppCommand + Send>>,
                                  Receiver<Box<AppCommand + Send>>) = mpsc::channel();

    let (_reader_tx, reader_rx) = mpsc::channel();

    let pri = PacketReaderServer::new(tx, app_tx, reader_rx);

    // Initialize our packet reader
    let _rthread = read_packets(pri, &listen_address);

    println!("Initialized.");

    let mut previous = get_current_time();

    let mut lag: f64 = 0.0;

    loop {
        let current = get_current_time();
        let elapsed = current - previous;
        previous = current;

        lag = lag + elapsed as f64;

        while lag >= MS_PER_UPDATE {
            // Process connection commands
            let _ = check_comm_commands(&command_rx, &mut client_state);
            let _ = check_app_commands(&app_command_rx);
            // Process game state

            // Crank engine
        }
        // render(lag / MS_PER_UPDATE) // Not useful for connection state but will be with graphics
    }
}

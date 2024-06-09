mod memtable;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use clap::{Arg, Command};
use crate::memtable::MemTable;


fn client_handler() {
    println!("Handling request")
}

fn main() {

    let matches = Command::new("ninja-kv-store")
        .version("0.1")
        .author("Shubham Tomar, shubhamtomar1498@gmail.com")
        .about("A simple KV store based LSM principles")
        .arg(Arg::new("memtable-max-size")
            .long("memtable-max-size")
            .default_value("1000")
            .help("Maximum size of the MemTable before flushing"))
        .get_matches();

    let memtable_max_size = matches.get_one::<String>("memtable-max-size")
        .unwrap_or(&"1000".to_string())
        .parse::<usize>()
        .unwrap_or(1000);
    let mem_table = Arc::new(Mutex::new(MemTable::new(memtable_max_size)));

}
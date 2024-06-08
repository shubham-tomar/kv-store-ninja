mod memtable;

use clap::{Arg, Command};
use crate::memtable::MemTable;

fn main() {
    let matches = Command::new("Key-Value Store")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("A simple key-value store using LSM tree principles")
        .subcommand(Command::new("put")
            .about("Inserts a key-value pair")
            .arg(Arg::new("key")
                .help("The key to insert")
                .required(true)
                .index(1))
            .arg(Arg::new("value")
                .help("The value to insert")
                .required(true)
                .index(2)))
        .subcommand(Command::new("get")
            .about("Retrieves a value by key")
            .arg(Arg::new("key")
                .help("The key to retrieve")
                .required(true)
                .index(1)))
        .subcommand(Command::new("delete")
            .about("Deletes a key")
            .arg(Arg::new("key")
                .help("The key to delete")
                .required(true)
                .index(1)))
        .subcommand(Command::new("scan")
            .about("Scans a range of keys")
            .arg(Arg::new("start")
                .help("The start key of the range")
                .required(true)
                .index(1))
            .arg(Arg::new("end")
                .help("The end key of the range")
                .required(true)
                .index(2)))
        .get_matches();

    let mut mem_table = MemTable::new(1000);

    if let Some(matches) = matches.subcommand_matches("put") {
        let key = matches.get_one::<String>("key").unwrap().as_bytes().to_vec();
        let value = matches.get_one::<String>("value").unwrap().as_bytes().to_vec();
        mem_table.put(key, value);
        println!("Inserted key-value pair.");
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.get_one::<String>("key").unwrap().as_bytes().to_vec();
        match mem_table.get(&key) {
            Some(entry) if !entry.deleted => println!("Value: {:?}", entry.value),
            _ => println!("Key not found or marked as deleted."),
        }
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let key = matches.get_one::<String>("key").unwrap().as_bytes().to_vec();
        mem_table.delete(&key);
        println!("Key deleted.");
    }

    // if let Some(matches) = matches.subcommand_matches("scan") {
    //     let start = matches.get_one::<String>("start").unwrap().as_bytes().to_vec();
    //     let end = matches.get_one::<String>("end").unwrap().as_bytes().to_vec();
        // let scan_result = mem_table.range_scan(&start, &end);
        // for (key, entry) in scan_result {
        //     if !entry.deleted {
        //         println!("{:?}: {:?}", key, entry.value);
        //     } else {
        //         println!("{:?} is deleted", key);
        //     }
        // }
    // }
}

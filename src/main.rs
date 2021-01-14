use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read cmdline args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: px86 filename");
        process::exit(1);
    }
    let filename = &args[1];

    // open file
    let mut file = File::open(filename)
        .expect("file not found");

    // read file as byte
    let mut memory: Vec<u8> = Vec::new();
    file.read_to_end(&mut memory)
        .expect("something went wrong reading the file");

    println!("Contents:\n {:02x?}", &memory);
}

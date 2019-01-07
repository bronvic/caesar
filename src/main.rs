use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

mod coder;
use coder::*;

#[derive(PartialEq)]
enum Mode {
    Encode,
    Decode,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        panic!("Wrong number of arguments\nUsage: {:?}\n\tpath to input file\n\t\"decode\" or \"encode\" flag\n\tshift as number\n", args[0]);
    }

    let relative_path = PathBuf::from(&args[1]);
    let mut path = std::env::current_dir().unwrap();
    path.push(relative_path);
    println!("path {}", path.display());
    let display = path.display();

    let mode: Mode;
    match args[2].as_ref() {
        "encode" => mode = Mode::Encode,
        "decode" => mode = Mode::Decode,
        val => panic!("Unrecognized argument \"{}\". Should be \"encode\" or \"decode\" instead", val)
    }

    let shift = (args[3].parse::<u32>().unwrap() % ALPHABET_SIZE as u32) as u8;


    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {},
    };

    if mode == Mode::Encode {
        println!("{}", encode(content, shift));
    } else if mode == Mode::Decode {
        println!("{}", decode(content, shift));
    }
}

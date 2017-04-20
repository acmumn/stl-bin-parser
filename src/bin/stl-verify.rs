extern crate stl_parser;

use std::env::args;
use std::fs::File;
use std::io::{Read, stderr, Write};
use std::process::exit;
use stl_parser::Stl;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Usage: stl-verify <stl-file>");
    }

    let mut file = File::open(&args[1]).expect("Couldn't open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Couldn't read file");

    match Stl::parse(&buf) {
        Ok(stl) => println!("Valid STL file with {} triangles.", stl.triangles.len()),
        Err(err) => {
            writeln!(stderr(), "{}", err).unwrap();
            exit(-1);
        }
    }
}

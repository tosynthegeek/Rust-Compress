extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compress;
use flate2::Compression;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::process::Output;
use std::time::Instant;

fn main() {
    /// Check if the command-line arguments is correct
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    /// Use BufReader to open the source file for reading.
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    /// Use File::create to create the target file
    let output = File::create(args().nth(2).unwrap()).unwrap();

    /// Use GzEncoder to set compression level to default
    let mut encoder = GzEncoder::new(output, Compression::default());

    /// Measuring compression time
    let start = Instant::now();

    /// Copy the content from the input file, compresss it and paste to the target file
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}

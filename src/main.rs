use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let fname = PathBuf::from(args().nth(1).ok_or("Expected filename")?);
    let buffread = BufReader::new(File::open(&fname)?);

    let total: i32 = buffread.lines()
        .filter_map(|line| line.ok()?.parse::<i32>().ok())
        .sum();

    println!("Total: {}", total);
    Ok(())
}

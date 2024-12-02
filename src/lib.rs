use std::fmt::Display;
use std::io;
use std::io::prelude::*;

pub fn read_in() -> Result<Vec<String>, io::Error> {
    let mut input = Vec::new();
    for l in io::stdin().lock().lines() {
        input.push(l?);
    }
    Ok(input)
}

pub fn write_out<T: Display>(s: T) {
    println!("{}", s);
}

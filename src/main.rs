//use windows_args::Args;
use std::collections;
use std::io;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{de,ser};
use subprocess::Exec;
 

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    program : String,
    args    : Vec<String>,
    delay   : i32,
    message : String
}


fn main() {
    let mut argv: Vec<String>;


    println!("Hello, world!");
}

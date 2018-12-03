use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq)]
pub enum ProblemPart { A, B }

pub fn input(filenum: &str) -> String {
    let mut filename = String::from("./input/input");
    filename.push_str(filenum);
    filename.push_str(".txt");
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Remove trailing newline
    contents.pop();

    contents
}

extern crate regex;

use regex::Regex;

fn main() {

    println!("Hello, world!");

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}

// fn hello(name: &str) {
//     if 1 == 1 {
//        // println!("nihao " & name)
//     }
// }
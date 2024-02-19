#![allow(unused)]

use std::io;

fn main(){
    println!("What's your name?");

    let mut greeting = "Hello, Mr.";
    let mut name = String::new();

    io::stdin().read_line(&mut name)
    .expect("Somthing went wrong!");

    let binding = name.to_string();
    let parsed_name = binding.trim_end();

    if parsed_name.is_empty() {
        println!("Didn't enter any name");
    } else {
        println!("{}{}", greeting, parsed_name.to_uppercase());
    }
}
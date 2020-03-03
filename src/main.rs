// Dependencies
#[allow(unused_imports)]
use std::io::stdin;
#[allow(unused_imports)]
use colored::Colorize;

// Main
#[allow(unused_variables)]
fn main() {
    /*
    // Colors
    let red = "\x1b[31;1m";
    let green = "\x1b[32;1m";
    let yellow = "\x1b[33;1m";
    let blue = "\x1b[34;1m";
    let magenta = "\x1b[35;1m";
    let cyan = "\x1b[36;1m";
*/

    let mut input = String::new(); // Creates new string

    println!("{}", "How was your day mate?");
    
    stdin().read_line(&mut input) // Reads the string
        .ok()
        .expect("Failed to read line"); // Error handling

    // Interpret input
    match &input.trim().to_lowercase()[..] {
        "good" => println!("Awesome!"),
        _ => println!("Something else"),
    };

/*
    // Loops
    let mut i = 0;
    for j in 1..12 {
        println!("{}{}", blue, i);
        i += 1;
    }
*/
}
// Dependencies
use std::io::stdin;
use runas::Command;
use std::process;

// Main
#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {

    // Variables

    // Colors
    let red = "\x1b[31;1m";
    let green = "\x1b[32;1m";
    let yellow = "\x1b[33;1m";
    let blue = "\x1b[34;1m";
    let magenta = "\x1b[35;1m";
    let cyan = "\x1b[36;1m";
    let reset = "\x1b[36;1m";

    let mut input = String::new(); // Creates new string

    println!("{}", "How was your day mate?");
    
    stdin().read_line(&mut input) // Reads the string
        .ok()
        .expect("Failed to read line"); // Error handling

    // Interpret input
    match &input.trim()[..] {
        "0x20" => println!("{}Blank!", yellow),
        "good" => println!("{}Awesome!", green),
        _ => println!("{}Something else", red),
    };

    // Loops
    for i in 1..=5 {
        println!("{}{}{}", blue, i, reset);
    }

    // Run sudo system commands
    Command::new("cat")
    .arg("/root/.bashrc")
    .status();

    // Run user system commands
    process::Command::new("cat")
    .arg("/home/inc0gnit0/.bashrc")
    .status();
}
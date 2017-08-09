extern crate project_euler;
use project_euler::*;
use std::io::{self, Write};
use std::env;

fn main() {
    // get problem number
    let mut input = String::new();
    if env::args().len() > 1 {
        input = env::args().nth(1).unwrap();
    } else {
        io::stdout().write(b"Problem #: ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.pop().unwrap(); // remove newline character
    }
    if let Ok(numb) = input.parse::<u32>() {
        //println!("{}", numb);
        get_problem(numb);
    } else {
        println!("{} is not a valid question number.", input);
    }
}

// add entry to here when a problem is complete.
fn get_problem(n: u32) {
    match n {
        1 => { q_1::main(); },
        2 => { q_2::main(); },
        3 => { q_3::main(); },
        4 => { q_4::main(); },
        100 => { q_100::main(); },
        _ => { println!("Question {} not yet solved.", n); },
    }
}


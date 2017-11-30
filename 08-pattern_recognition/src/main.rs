#![feature(io)]
use std::io::{stdin, BufRead, BufReader, Read};
use std::fs::File;

// fn get_input() -> String {
//     let mut buffer = String::new();
//     let stdin = stdin();
//     stdin
//         .lock()
//         .read_line(&mut buffer)
//         .expect("Could not read line");
//     buffer
// }

enum State {
    A,
    B,
    C,
    D,
    E
}

use State::*;

fn main() {
    let path = "../res/in/hw08_01";
    let input = File::open(path).expect("Unable to open");
    let buffer = BufReader::new(input);

    let mut counter = 0;
    let mut current_state = A;

    let mut range = (0, 0);

    for (i, c) in buffer.chars().enumerate() {
        let c = c.unwrap();
        // println!("'{}'", c);

        match current_state {
            A => {
                range.0 = i + 1;
                current_state = if c == '1' { B } else { A };
            }
            B => {
                current_state = if c == '1' { C } else { A };
            }
            C => {
                current_state = if c == '0' { D } else { C };
            }
            D => if c == '1' {
                current_state = E;
                counter += 1;
                range.1 = i + 1;
                println!("Found pattern from: {:?}", range);
            } else {
                current_state = A;
            }
            E => if c == '1' {
                current_state = C;
                range.0 = i;
            } else {
                current_state = A;
            }
        }
    }

    println!("Counter: {}", counter);
}

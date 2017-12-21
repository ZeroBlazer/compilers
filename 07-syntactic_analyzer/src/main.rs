extern crate ansi_term;

use std::io::{stdin, BufRead};
use ansi_term::Colour::*;

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = stdin();
    stdin
        .lock()
        .read_line(&mut buffer)
        .expect("Could not read line");
    buffer
}

#[derive(Debug, Clone, PartialEq)]
enum States {
    S,
    A,
    B,
    C,
    Error,
    Ok,
}

use States::*;

fn eval(input: &str) {
    let mut col = 1;
    let mut input_chars = input.chars();
    let mut get_char = || input_chars.next().unwrap();

    let mut preanalisis = get_char();
    let mut curr_state = S;
    let mut next_state = (Error, Error);

    while { input.len() - col > 0 } {
        // println!(">> {:?}: {}", curr_state, preanalisis);

        match curr_state {
            S => if preanalisis == 'x' {
                preanalisis = get_char();
                col += 1;
                curr_state = S;
            } else if preanalisis == 'a' {
                curr_state = A;
            } else {
                curr_state = Error;
                next_state = (S, B);
            },
            A => if preanalisis == 'a' {
                preanalisis = get_char();
                col += 1;
                curr_state = B;
            } else {
                curr_state = Error;
                next_state = (A, B);
            },
            B => if preanalisis == 'b' {
                preanalisis = get_char();
                col += 1;
                curr_state = C;
            } else {
                curr_state = Error;
                next_state = (B, C);
            },
            C => if preanalisis == 'c' {
                curr_state = Ok;
            } else {
                curr_state = Error;
                next_state = (C, Error);
            },
            Error => {
                print!("\n{}", Red.bold().paint("Error de sintáxis"));
                println!(": {:?} >> {:?}", next_state, curr_state);
                print!("{}", Blue.bold().paint(format!("  En columna {}, el caracter '{}' debería ser: ", col, preanalisis)));

                match next_state.0 {
                    S => println!("{}", Yellow.bold().paint("'x'/'a'")),
                    A => println!("{}", Yellow.bold().paint("'a'")),
                    B => println!("{}", Yellow.bold().paint("'b'")),
                    C => println!("{}", Yellow.bold().paint("'c'")),
                    _ => {
                        println!("{}", Yellow.bold().paint("''"));
                        break;
                    }
                }

                curr_state = next_state.1.clone();
                preanalisis = get_char();
                col += 1;
            }
            Ok => {
                println!("{}", Green.bold().italic().paint("Success!"));
                break;
            }
        }
    }

    if curr_state != Ok {
        println!("\n{}: Patrón sin terminar", Red.bold().paint("Error de sintáxis"));
    }
}

fn main() {
    let input = get_input();
    input.trim();
    println!("'{}'", input);
    eval(&input);
}

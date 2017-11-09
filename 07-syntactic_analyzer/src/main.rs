use std::io::{stdin, BufRead};

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = stdin();
    stdin
        .lock()
        .read_line(&mut buffer)
        .expect("Could not read line");
    buffer
}

// fn s_rule(input_chars: &mut std::str::Chars) {
//     let mut get_char = || {
//         input_chars.next().unwrap()
//     };

//     let mut preanalisis = get_char();
//     //***************Closures***************//
//     let error = || {
//         println!("Error de Sintáxis");
//     };

//     // let mut parea = |t| {
//     //     if preanalisis == t {
//     //         preanalisis = get_char();
//     //     } else {
//     //         error();
//     //     }
//     // };

//     if preanalisis == 'x' {
//         s_rule(input_chars);
//     }
// }

#[derive(Debug)]
enum States {
    S,
    A,
    B,
    Error,
    Ok
}

use States::*;

fn eval(input_chars: &mut std::str::Chars) {
    let mut get_char = || input_chars.next().unwrap();

    let mut preanalisis = get_char();
    let mut curr_state = S;

    for _ in 0..5 {
        // println!("{:?}: {}", curr_state, preanalisis);
        match curr_state {
            S => if preanalisis == 'x' {
                preanalisis = get_char();
                curr_state = S;
            } else if preanalisis == 'a' {
                curr_state = A;
            } else {
                curr_state = Error;
            },
            A => if preanalisis == 'a' {
                preanalisis = get_char();
                curr_state = B;
            } else {
                curr_state = Error;
            },
            B => if preanalisis == 'b' {
                preanalisis = get_char();
                if preanalisis == 'c' {
                    curr_state = Ok;
                } else {
                    curr_state = Error;
                }
            } else {
                curr_state = Error
            },
            Error => {
                println!("Error de sintáxis");
                println!("En columna: {}", input_chars
            }
            Ok => println!("Success!"),
        }
    }
}

fn main() {
    let input = get_input();

    let mut preanalisis = input.chars();
    eval(&mut preanalisis);
}

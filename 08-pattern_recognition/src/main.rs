#![feature(io)]
use std::io::{stdin, BufRead, BufReader, Read};
use std::fs::File;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::Vacant;

// fn get_input() -> String {
//     let mut buffer = String::new();
//     let stdin = stdin();
//     stdin
//         .lock()
//         .read_line(&mut buffer)
//         .expect("Could not read line");
//     buffer
// }

// use State::*;

fn get_matrix(pattern: &str, transitions: &[char]) -> BTreeMap<usize, BTreeMap<char, usize>> {
    let n_states = pattern.len() + 1;

    let mut matrix: BTreeMap<usize, BTreeMap<char, usize>> = BTreeMap::new();

    let init_trans = pattern.chars().nth(0).unwrap();
    let mut break_pos = 1;
    {
        let mut prev_char = pattern.chars().nth(0).unwrap();
        for c in pattern.chars().skip(1) {
            if c != prev_char {
                break;
            } else {
                break_pos += 1;
            }
        }
    }

    (0..n_states).for_each(|state| {
        let inputs = matrix.entry(state).or_insert_with(BTreeMap::new);
        let pro_trans = pattern.chars().nth(if state != n_states - 1 { state } else { state - 1 }).unwrap(); // Progress trans

        if state != n_states - 1 {
            inputs.insert(pro_trans, state + 1);
            if state == break_pos {
                transitions.iter().filter(|t| **t != pro_trans).for_each(|t| {
                    inputs.insert(*t, break_pos);
                });
            }
        } else {
            if pro_trans == init_trans {
                inputs.insert(pro_trans, break_pos);
            }
        }

        transitions
            .iter()
            .for_each(|t| { inputs.entry(*t).or_insert(0); });
    });

    println!("{:#?}", matrix);
    matrix
}

fn main() {
    // let path = "../res/in/hw08_01";
    // let input = File::open(path).expect("Unable to open");
    // let buffer = BufReader::new(input);

    let pattern = "1101";
    let transitions = ['0', '1'];
    
    let matrix = get_matrix(pattern, &transitions);

    let mut counter = 0;
    // let current_state = 0;
    // let mut range = (0, 0);

    // for (i, c) in buffer.chars().enumerate() {
    //     let c = c.unwrap();
    //     // println!("'{}'", c);

    //     // match counter {

    //     // }
    // }

    println!("Counter: {}", counter);
}

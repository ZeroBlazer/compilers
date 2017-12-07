#![feature(io)]
use std::io::{BufReader, Read};
use std::fs::File;
use std::collections::BTreeMap;

fn get_matrix(pattern: &str, transitions: &[char]) -> BTreeMap<usize, BTreeMap<char, usize>> {
    let n_states = pattern.len() + 1;

    let mut matrix: BTreeMap<usize, BTreeMap<char, usize>> = BTreeMap::new();

    let init_trans = pattern.chars().nth(0).unwrap();
    let mut break_pos = 1;

    for c in pattern.chars().skip(1) {
        if c != init_trans {
            break;
        } else {
            break_pos += 1;
        }
    }

    (0..n_states).for_each(|state| {
        let inputs = matrix.entry(state).or_insert_with(BTreeMap::new);
        let pro_trans = pattern
            .chars()
            .nth(if state != n_states - 1 {
                state
            } else {
                state - 1
            })
            .unwrap(); // Progress trans

        if state != n_states - 1 {
            inputs.insert(pro_trans, state + 1);
            if state == break_pos {
                transitions
                    .iter()
                    .filter(|t| **t != pro_trans)
                    .for_each(|t| {
                        inputs.insert(*t, break_pos);
                    });
            }
        } else if pro_trans == init_trans {
            inputs.insert(pro_trans, break_pos);
        }

        transitions.iter().for_each(|t| {
            inputs.entry(*t).or_insert(0);
        });
    });

    println!("{:#?}", matrix);
    matrix
}

fn main() {
    let path = "../res/in/hw08_01";
    let input = File::open(path).expect("Unable to open");
    let buffer = BufReader::new(input);

    // let pattern = "1101";
    let pattern = "101";
    let transitions = ['0', '1'];

    let matrix = get_matrix(pattern, &transitions);

    let mut counter = 0;
    let mut current_state = 0;
    let final_state = matrix.len() - 1;
    // let mut range = (0, 0);

    for c in buffer.chars() {
        let c = c.unwrap();
        // println!("({}, {})", i, c);

        if current_state == final_state {
            counter += 1;
        }

        current_state = matrix[&current_state][&c];
    }

    println!("Counter: {}", counter);
}

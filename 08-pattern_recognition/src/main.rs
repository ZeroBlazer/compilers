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

fn main() {
    let path = "../res/in/hw08_01";
    let input = File::open(path).expect("Unable to open");
    let buffer = BufReader::new(input);

    let pattern = "1101";
    let transtitions = ['0', '1'];
    let n_states = pattern.len() + 1;

    let mut matrix: BTreeMap<usize, BTreeMap<char, usize>> = BTreeMap::new();

    // (0..n_states).for_each(|x| {
    //     matrix.insert(x, BTreeMap::new());
    // });

    let initial_trans = pattern.chars().nth(0).unwrap();

    (0..n_states).for_each(|state| {
        let inputs = matrix.entry(state).or_insert(BTreeMap::new());
        
        if state != n_states - 1 {
            inputs.insert(pattern.chars().nth(state).unwrap(), state + 1);
        }

        transtitions.iter().for_each(|t| {
            inputs.entry(*t).or_insert(if *t == initial_trans {
                matrix[&0][&initial_trans]
            } else {
                0
            });
        });
    });

    println!("{:#?}", matrix);

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

extern crate thompson;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use thompson::*;

fn l_closure(automata: &AutomataFN, state: u32) -> BTreeSet<u32> {
    let mut nodes = BTreeSet::new();
    let mut buf = VecDeque::new();
    buf.push_back(state);

    while !buf.is_empty() {
        let state = buf.pop_front().unwrap();
        nodes.insert(state);

        for trans in &automata.transitions {
            if trans.node_from == state && trans.trans_sym == 'λ' {
                buf.push_back(trans.node_to);
            }
        }
    }

    nodes
}

fn delta(automata: &AutomataFN, states: &BTreeSet<u32>, entry: char) -> BTreeSet<u32> {
    let mut nodes = BTreeSet::new();

    for state in states {
        for trans in &automata.transitions {
            if trans.node_from == *state && trans.trans_sym == entry {
                nodes.insert(trans.node_to);
            }
        }
    }

    nodes
}

fn l_closure_set(automata: &AutomataFN, states: &BTreeSet<u32>) -> BTreeSet<u32> {
    let mut nodes = BTreeSet::new();

    for state in states {
        nodes.append(&mut l_closure(&automata, *state));
    }

    nodes
}

#[derive(Debug)]
pub struct AutomataFD {
    expr: String,
    states: BTreeMap<u32, BTreeSet<u32>>,
    initial_state: u32,
    accept_states: Vec<u32>,
    entries: BTreeSet<char>,
    pub transitions: Vec<Transition>,
}

pub fn subset_construction(in_path: &str) {
    let autom_fn = thompson(in_path);
    let mut counter = 0;
    let mut states = BTreeMap::new();

    let initial_state = counter;
    counter += 1;
    // states.insert(initial_state, l_closure(&autom_fn, autom_fn.initial_state));

    println!("{:?}", l_closure_set(&autom_fn, &delta(&autom_fn, &l_closure(&autom_fn, autom_fn.initial_state), 'b')));

    let autom_fd = AutomataFD {
        expr: autom_fn.expr,
        states: states,
        initial_state: initial_state,
        accept_states: vec![1, 2],
        entries: autom_fn.entries,
        transitions: Vec::new() // autom_fn.transitions
    };

    // println!("{:#?}", autom_fd);
}
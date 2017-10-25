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

impl AutomataFD {
    pub fn display(&self) {
        println!("\nEXPRESIÓN: {}", self.expr);
        println!("ESTADOS:");
        for (state, eq_states) in &self.states {
            println!("{}: {:?}", state, eq_states);
        }
        println!("---------------------------------------");
        println!(
            "INICIAL:\n{}\n---------------------------------------",
            &self.initial_state
        );
        println!(
            "ACEPTACIÓN:\n{:?}\n---------------------------------------",
            &self.accept_states
        );
        println!(
            "ENTRIES:\n{:?}\n---------------------------------------",
            &self.entries
        );
        println!("TRANSICIONES:");
        for trans in &self.transitions {
            println!("{}", trans);
        }
        println!("=======================================");
    }
}

pub fn subset_construction(in_path: &str) {
    let autom_fn = thompson(in_path);
    let mut counter = 1;
    let mut marked_states = BTreeMap::new();
    let mut dfa_states = BTreeMap::new();

    let initial_state = counter;
    counter += 1;

    autom_fn.display();

    let initial_dfa_state = l_closure(&autom_fn, autom_fn.initial_state);
    // println!("Initial: {:?}", initial_dfa_state);
    marked_states.insert(initial_dfa_state.clone(), false);
    dfa_states.insert(initial_state, initial_dfa_state);

    loop {
        if 
    }
    // for entry in &autom_fn.entries {
    //     let l_clos = l_closure(&autom_fn, autom_fn.initial_state);
    //     let delta_p = l_closure_set(&autom_fn, &delta(&autom_fn, &l_clos, *entry));
    //     if !delta_p.is_empty() && !marked_states.contains(&delta_p) {
    //         marked_states.insert(delta_p.clone());
    //         dfa_states.insert(counter, delta_p);
    //         counter += 1;
    //     }
    //     println!("States: {:?}", marked_states);
    // }

    let autom_fd = AutomataFD {
        expr: autom_fn.expr,
        states: dfa_states,
        initial_state: initial_state,
        accept_states: vec![1, 2],
        entries: autom_fn.entries,
        transitions: Vec::new(), // autom_fn.transitions
    };

    autom_fd.display();
}

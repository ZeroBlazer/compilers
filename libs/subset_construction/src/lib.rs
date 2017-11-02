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

fn d_transitions(automata: &AutomataFN, states: &BTreeSet<u32>, entry: char) -> BTreeSet<u32> {
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
    states: BTreeMap<BTreeSet<u32>, u32>,
    initial_state: u32,
    accept_states: Vec<u32>,
    entries: BTreeSet<char>,
    pub transitions: Vec<Transition>,
}

impl AutomataFD {
    pub fn display(&self) {
        println!("\n==================AFD==================");
        println!("EXPRESIÓN: {}", self.expr);
        println!("ESTADOS:");
        for (eq_states, state) in &self.states {
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
    autom_fn.display();

    let mut new_id = 1;
    let initial_state = new_id;
    let mut dfa_states = BTreeMap::new();
    let mut marked_states = BTreeMap::new();
    let mut transitions = Vec::new();

    let mut r_state = l_closure(&autom_fn, autom_fn.initial_state);
    dfa_states.insert(r_state.clone(), new_id);
    marked_states.insert(new_id, false);
    new_id += 1;

    let mut found = false;
    // for _ in 0..5 {
    while !found {
        let mut t_state = r_state.clone();
        let mut t_state_id = 0;
        for (state, state_id) in &dfa_states {
            if !marked_states[state_id] {
                t_state = state.clone();
                t_state_id = *state_id;
                found = true;
            }
        }

        if found {
            if let Some(mark) = marked_states.get_mut(&t_state_id) {
                *mark = true;
            }

            for entry in &autom_fn.entries {
                r_state = l_closure_set(&autom_fn, &d_transitions(&autom_fn, &t_state, *entry));
                if !r_state.is_empty() {
                    if !dfa_states.contains_key(&r_state) {
                        dfa_states.insert(r_state.clone(), new_id);
                        marked_states.insert(new_id, false);
                        new_id += 1;
                    }
                    transitions.push(Transition::new(t_state_id, *entry, dfa_states[&r_state]));
                }
            }

            found = false;
        } else {
            break;
        }
    }

    let mut accept_states = Vec::new();
    for accept_state in autom_fn.accept_states {
        for (state, state_id) in &dfa_states {
            if state.contains(&accept_state) {
                accept_states.push(*state_id);
            }
        }
    }

    let autom_fd = AutomataFD {
        expr: autom_fn.expr,
        states: dfa_states,
        initial_state: initial_state,
        accept_states: accept_states,
        entries: autom_fn.entries,
        transitions: transitions,
    };

    autom_fd.display();
}

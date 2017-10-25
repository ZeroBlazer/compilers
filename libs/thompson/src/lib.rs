// #[macro_use]
// extern crate serde_derive;
// extern crate serde;
extern crate posfixer;

use posfixer::*;
use posfixer::TermType::*;
use std::fmt;
use std::io::BufRead;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Transition {
    pub node_from: u32,
    pub trans_sym: char,
    pub node_to: u32,
}

impl Transition {
    fn new(node_from: u32, trans_sym: char, node_to: u32) -> Transition {
        Transition {
            node_from: node_from,
            trans_sym: trans_sym,
            node_to: node_to,
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} -> {}",
            self.node_from,
            self.trans_sym,
            self.node_to
        )
    }
}

#[derive(Debug, Clone)]
pub struct AutomataFN {
    pub expr: String,
    states: Vec<u32>,
    pub initial_state: u32,
    accept_states: Vec<u32>,
    pub entries: BTreeSet<char>,
    pub transitions: Vec<Transition>,
}

impl AutomataFN {
    fn new(entry: char, state1: u32, state2: u32) -> AutomataFN {
        AutomataFN {
            expr: entry.to_string(),
            states: vec![state1, state2],
            initial_state: state1,
            accept_states: vec![state2],
            entries: [entry].iter().cloned().collect(),
            transitions: vec![Transition::new(state1, entry, state2)],
        }
    }

    pub fn display(&self) {
        println!("\nEXPRESIÓN: {}", self.expr);
        println!(
            "ESTADOS:\n{:?}\n---------------------------------------",
            &self.states
        );
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

    // pub fn transitions(&self) -> &[Transition] {
    //     &self.transitions[..]
    // }
}

fn new_automata_fn(counter: &mut u32, entry: char) -> AutomataFN {
    *counter += 2;
    AutomataFN::new(entry, *counter - 2, *counter - 1)
}

fn concat(auto1: AutomataFN, mut auto2: AutomataFN) -> AutomataFN {
    let expr = auto1.expr + " " + auto2.expr.as_ref() + " .";
    let mut states = auto1.states;
    states.append(&mut auto2.states);
    let accept_states = auto2.accept_states;
    let mut entries = auto1.entries;
    entries.append(&mut auto2.entries);
    let mut transitions = auto1.transitions;
    transitions.append(&mut auto2.transitions);

    for accept in &auto1.accept_states {
        transitions.push(Transition::new(*accept, 'λ', auto2.initial_state));
    }

    AutomataFN {
        expr: expr,
        states: states,
        initial_state: auto1.initial_state,
        accept_states: accept_states,
        entries: entries,
        transitions: transitions,
    }
}

fn alternative(auto1: AutomataFN, mut auto2: AutomataFN, counter: &mut u32) -> AutomataFN {
    let new_state1 = *counter;
    *counter += 1;
    let new_state2 = *counter;
    *counter += 1;

    let expr = auto1.expr + " " + auto2.expr.as_ref() + " +";
    let mut states = auto1.states;
    states.append(&mut auto2.states);
    states.push(new_state1);
    states.push(new_state2);
    let mut entries = auto1.entries;
    entries.append(&mut auto2.entries);
    let mut transitions = auto1.transitions;
    transitions.append(&mut auto2.transitions);

    transitions.push(Transition::new(new_state1, 'λ', auto1.initial_state));
    transitions.push(Transition::new(new_state1, 'λ', auto2.initial_state));

    for accept in &auto1.accept_states {
        transitions.push(Transition::new(*accept, 'λ', new_state2));
    }

    for accept in &auto2.accept_states {
        transitions.push(Transition::new(*accept, 'λ', new_state2));
    }

    AutomataFN {
        expr: expr,
        states: states,
        initial_state: new_state1,
        accept_states: vec![new_state2],
        entries: entries,
        transitions: transitions,
    }
}

fn kleine(auto: AutomataFN, counter: &mut u32) -> AutomataFN {
    let new_state1 = *counter;
    *counter += 1;
    let new_state2 = *counter;
    *counter += 1;

    let expr = auto.expr + " *";
    let mut states = auto.states;
    states.push(new_state1);
    states.push(new_state2);
    let mut transitions = auto.transitions;

    for accept in &auto.accept_states {
        transitions.push(Transition::new(*accept, 'λ', auto.initial_state));
        transitions.push(Transition::new(*accept, 'λ', new_state2));
    }

    transitions.push(Transition::new(new_state1, 'λ', auto.initial_state));
    transitions.push(Transition::new(new_state1, 'λ', new_state2));

    AutomataFN {
        expr: expr,
        states: states,
        initial_state: new_state1,
        accept_states: vec![new_state2],
        entries: auto.entries,
        transitions: transitions,
    }
}

pub fn thompson(in_path: &str) -> AutomataFN {
    posfix(in_path, "../res/out/posfija");

    let mut counter = 0;
    let posfixed = get_file_buffer("../res/out/posfija");
    let line = posfixed.lines().next().unwrap().unwrap();
    let terms: Vec<&str> = line.split_whitespace().collect();

    let mut heap = Vec::new();

    for term in &terms {
        match eval_term(term) {
            Number | Variable => {
                heap.push(new_automata_fn(&mut counter, term.chars().nth(0).unwrap()));
            }
            Operator => {
                match *term {
                    "*" => {
                        let autom = kleine(heap.pop().unwrap(), &mut counter);

                        heap.push(autom);
                    }
                    "." => {
                        let autom2 = heap.pop().unwrap();
                        let autom1 = heap.pop().unwrap();

                        let autom = concat(autom1, autom2);

                        heap.push(autom);
                    }
                    "+" => {
                        let autom2 = heap.pop().unwrap();
                        let autom1 = heap.pop().unwrap();

                        let autom = alternative(autom1, autom2, &mut counter);

                        heap.push(autom);
                    }
                    _ => {}
                }
                // heap.last().unwrap().display();  // DISPLAY AUTOMATA AFTER COMPUTING OPERATOR
            }
            _ => {}
        }
    }

    // println!("{:#?}", heap)
    heap.pop().unwrap()
}

extern crate posfixer;

use posfixer::*;
use posfixer::TermType::*;
use std::fmt;
use std::io::BufRead;

#[derive(Debug)]
struct Transition {
    node_from: u32,
    trans_sym: char,
    node_to: u32
}

impl Transition {
    fn new(node_from: u32, trans_sym: char, node_to: u32) -> Transition {
        Transition {
            node_from: node_from,
            trans_sym: trans_sym,
            node_to: node_to
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} -> {}", self.node_from, self.trans_sym, self.node_to)
    }
}

struct AutomataFN {
    expr: String,
    states: Vec<u32>,
    initial_state: u32,
    accept_states: Vec<u32>,
    entries: Vec<char>,
    transitions: Vec<Transition>
}

impl AutomataFN {
    fn new(entry: char, state1: u32, state2: u32) -> AutomataFN {
        AutomataFN {
            expr: entry.to_string(),
            states: vec![state1, state2],
            initial_state: state1,
            accept_states: vec![state2],
            entries: vec![entry],
            transitions: vec![Transition::new(state1, entry, state2)]
        }
    }

    fn display(&self) {
        println!("\nEXPRESIÓN: {}", self.expr);
        println!("ESTADOS:\n{:?}\n---------------------------------------",
                 &self.states);
        println!("INICIAL:\n{}\n---------------------------------------",
                 &self.initial_state);
        println!("ACEPTACIÓN:\n{:?}\n---------------------------------------",
                 &self.accept_states);
        println!("ENTRIES:\n{:?}\n---------------------------------------",
                 &self.entries);
        println!("TRANSICIONES:");
        for trans in &self.transitions {
            println!("{}", trans);
        }
        println!("=======================================");
    }
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
        transitions: transitions
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
        transitions: transitions
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
        transitions: transitions
    }
}

fn thompson(in_path: &str) {
    posfix(in_path, "out/posfija");
    
    let mut counter = 0;
    let posfixed = get_file_buffer("out/posfija");
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
                        autom.display();
                        heap.push(autom);
                    }
                    "." => {
                        let autom2 = heap.pop().unwrap();
                        let autom1 = heap.pop().unwrap();

                        let autom = concat(autom1, autom2);

                        autom.display();
                        heap.push(autom);
                    }
                    "+" => {
                        let autom2 = heap.pop().unwrap();
                        let autom1 = heap.pop().unwrap();

                        let autom = alternative(autom1, autom2, &mut counter);

                        autom.display();
                        heap.push(autom);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() {
    // thompson("in/ex01");
    // thompson("in/ex02");
    thompson("in/hw05");
}

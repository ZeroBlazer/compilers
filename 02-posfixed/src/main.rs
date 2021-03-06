#![feature(type_ascription)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
enum TermType {
    Number,
    Variable,
    LeftGap,
    RightGap,
    Operator,
    Error,
}

use TermType::*;

fn get_file_buffer(path: &str) -> BufReader<File> {
    let input = File::open(path).expect("Unable to open");
    BufReader::new(input)
}

fn get_output_file(path: &str) -> File {
    File::create(path).expect("Couldn't open write file")
}

fn greater_eq_precedence(term1: &str, term2: &String) -> bool {
    if term1 == term2 {
        true
    } else {
        match term1 {
            "+" | "-" => {
                match term2.as_ref(): &str {
                    "+" | "-" => {
                        true
                    }
                    "*" | "/" => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            "*" | "/" => {
                match term2.as_ref(): &str {
                    "+" | "-" => {
                        false
                    }
                    "*" | "/" => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            "=" => {
                match term2.as_ref(): &str {
                    "+" | "-" => {
                        true
                    }
                    "*" | "/" => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            _ => false
        }
    }
}

fn eval_term(term: &str) -> TermType {
    match term.len() {
        0 => Error,
        1 => if term == "+" || term == "-" || term == "*" || term == "/" || term == "=" {
            Operator
        } else if term == "(" {
            LeftGap
        } else if term == ")" {
            RightGap
        } else if term.chars().nth(0).unwrap().is_numeric() {
            Number
        } else if term.chars().nth(0).unwrap().is_alphanumeric() {
            Variable
        } else {
            Error
        },
        _ => {
            let term = term.trim_matches('-');
            let mut numeric = true;
            for c in term.chars() {
                if !c.is_numeric() {
                    numeric = false;
                    break;
                }
            }

            if numeric {
                Number
            } else {
                Variable
            }
        }
    }
}

fn posfix(input: &mut BufReader<File>, output: &File) -> bool {
    let mut heap = Vec::new();
    let mut out = Vec::new();

    let line = input.lines().next().unwrap().unwrap();
    let terms: Vec<&str> = line.split_whitespace().collect();

    println!("{:?}", out);
    println!("{:?}", heap);

    for term in &terms {
        print!("{:?}: ", eval_term(term));
        match eval_term(term) {
            Number => {
                out.push(String::from(*term));
            }
            Variable => {
                out.push(String::from(*term));
            }
            LeftGap => {
                heap.push(String::from(*term));
            }
            RightGap => {
                while !heap.is_empty() && !heap.ends_with(&[String::from("(")]) {
                    out.push(heap.pop().unwrap());
                }
                if heap.ends_with(&[String::from("(")]) {
                    heap.pop();
                }
            }
            Operator => {
                while !heap.is_empty() && greater_eq_precedence(term, &heap.last().unwrap())  {
                    out.push(heap.pop().unwrap());
                }
                heap.push(String::from(*term));
            }
            _ => return false,
        }
        println!("{:?}", term);
        println!("{:?}", out);
        println!("{:?}", heap);
    }

    while !heap.is_empty()  {
        out.push(heap.pop().unwrap());
    }

    println!("{:?}", out);
    println!("{:?}", heap);

    true
}

fn main() {
    let mut file = get_file_buffer("../res/infija");
    let output = get_output_file("out/posfija");

    posfix(&mut file, &output);
    println!("Hello, world!");
}

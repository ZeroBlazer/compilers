#![feature(io)]
// // #![feature(exclusive_range_pattern)]

// enum Symbol {
// 	Exp,
// 	Resto,
// 	Term,
// }

// use Symbol::*;

// // fn recognize_segment(segment: &str) -> Symbol {
// // }

// fn get_next_symbol(symbol: Symbol, segment: &str) -> Vec<Symbol> {
// 	match symbol {
// 		Exp => vec![Term, Resto],
// 		Resto => {
// 			if segment == "+" || segment == "-" {
// 				vec![Term, Resto]
// 			}
// 			else {
// 				vec![]
// 			}
// 		}
// 		Term => {
// 			if segment.is_numeric() {
// 				vec![]
// 			} else {
// 				panic!("Error, input can't be a valid term")
// 			}
// 		}
// 	}
// }

// fn eval_exp() {
// 	let input = "9 + 5 - 3";
// 	let segments: Vec<&str> = input.split_whitespace().collect();

// 	let mut stack: Vec<Symbol> = vec![Exp];
// 	let mut it = segments.iter();

// 	let mut to_append = Vec::new();

// 	while {
// 		stack.append(&mut to_append);
// 		!stack.is_empty()
// 	} {
// 		let current_symbol = stack.pop().unwrap();
// 		let segment = it.next().unwrap();

// 		to_append = get_next_symbol(current_symbol, segment);
// 	}

//     println!("{:?}", segments);
// }

mod scanner;

use std::collections::VecDeque;
use scanner::TokenType;
use scanner::TokenType::*;

fn eval_exp(tokens: VecDeque<(TokenType, String)>) -> bool {
	if let Some(tokens) = eval_term(tokens) {
		if let Some(tokens) = eval_resto(tokens) {
			tokens.is_empty()
		} else {
			false
		} 
	} else {
		false
	}
}

fn eval_term(mut tokens: VecDeque<(TokenType, String)>) -> Option<VecDeque<(TokenType, String)>> {
	println!("{:?}", tokens[0]);

	if tokens[0].0 != NUM {
		None
	} else {
		tokens.pop_front();
		Some(tokens)
	}
}

fn eval_resto(mut tokens: VecDeque<(TokenType, String)>) -> Option<VecDeque<(TokenType, String)>> {
	if tokens.is_empty() {
		return Some(tokens)
	}

	println!("{:?}", tokens[0]);

	match tokens[0].0 {
		Mas | Menos => {
			tokens.pop_front();
			if let Some(tokens) = eval_term(tokens) {
				eval_resto(tokens)
			} else {
				None
			}
		}
		_ => Some(tokens)
	}
}

fn main() {
	// let input = "9 + 5 - 3";
	let input = "41333553";

	let tokens = scanner::token_scanner(&input);
	println!("{}", eval_exp(tokens));
	// eval_exp();
}

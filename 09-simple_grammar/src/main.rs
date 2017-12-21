#![feature(io)]

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
	if tokens[0].0 != NUM {
		None
	} else {
		print!("{} ", tokens.pop_front().unwrap().1);
		Some(tokens)
	}
}

fn eval_resto(mut tokens: VecDeque<(TokenType, String)>) -> Option<VecDeque<(TokenType, String)>> {
	if tokens.is_empty() {
		return Some(tokens)
	}

	match tokens[0].0 {
		Mas | Menos => {
			let tok = tokens.pop_front().unwrap();
			if let Some(tokens) = eval_term(tokens) {
				let ret = eval_resto(tokens);
				print!("{} ", tok.1);
				ret
			} else {
				None
			}
		}
		_ => Some(tokens)
	}
}

fn main() {
	// let input = "9 + 5 - 3";
	let input = "41333 + 553 + 321 - 6546 -654 +31654 -654645 -954654 65465";

	let tokens = scanner::token_scanner(input);
	
	if eval_exp(tokens) {
		println!("\nExpresión correcta");
	} else {
		println!("\nExpresión incorrecta");
	}
}

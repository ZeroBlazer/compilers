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
}

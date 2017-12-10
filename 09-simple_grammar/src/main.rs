// #![feature(exclusive_range_pattern)]

enum Symbol {
	Exp,
	Resto,
	Term,
}

use Symbol::*;

// fn recognize_segment(segment: &str) -> Symbol {
// }

fn get_next_symbol(symbol: Symbol, segment: &str) -> Vec<Symbol> {
	match symbol {
		Exp => vec![Term, Resto],
		Resto => {
			if segment == "+" || segment == "-" {
				vec![Term, Resto]
			}
			else {
				vec![]
			}
		}
		Term => {
			if segment.is_numeric() {
				vec![]
			} else {
				panic!("Error, input can't be a valid term")
			}
		}
	}
}

fn eval_exp() {
	let input = "9 + 5 - 3";
	let segments: Vec<&str> = input.split_whitespace().collect();

	let mut stack: Vec<Symbol> = vec![Exp];
	let mut it = segments.iter();

	let mut to_append = Vec::new();

	while {
		stack.append(&mut to_append);
		!stack.is_empty()
	} {
		let current_symbol = stack.pop().unwrap();
		let segment = it.next().unwrap();

		to_append = get_next_symbol(current_symbol, segment);
	}

    println!("{:?}", segments);
}

fn main() {
	eval_exp();
}

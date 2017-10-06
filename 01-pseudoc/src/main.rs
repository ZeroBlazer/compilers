use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
	let input = File::open("../res/pseudo.cod").expect("Unable to open");
	let mut output = File::create("out/main.cpp").expect("Couldn't open write file");

	writeln!(output, "#include <iostream>\nusing namespace std;\n").expect("Error writing in file");

	let file = BufReader::new(&input);
	let mut line_it = file.lines();

	{
		let mut line = line_it.next().unwrap().unwrap();
		if line.find("Inicio").is_some() {
			writeln!(output, "int main() {{").expect("Error writing in file");
		} else {
			panic!("Not valid syntax");
		}

		line = line_it.next().unwrap().unwrap();
		if line.find("Variables").is_some() {
			line = line_it.next().unwrap().unwrap();
			let mut vars: Vec<&str> = line.split(':').collect();
			let var_type = vars.pop().unwrap();

			// vars = vars[0].split(',').collect();
			vars = vars.iter().map(|v| v.trim()).collect();

			match var_type.trim() {
				"entero" => write!(output, "\tint ").expect("Error writing in file"),
				"real" => write!(output, "\tfloat ").expect("Error writing in file"),
				_ => write!(output, "\ttype ").expect("Error writing in file"),
			}

			writeln!(output, "{};", vars[0]).expect("Error writing in file");

		} else {
			panic!("Not valid syntax");
		}
	}

	for line in line_it {
		let line = line.unwrap();
		let statement: Vec<&str> = line.split_whitespace().collect();
		let keyword = statement[0];
		println!("{}", keyword);
		match keyword.trim() {
			"Leer" => {
				writeln!(output, "\tcin >> {};", statement[1]).expect("Error writing in file")
			}
			"Escribir" => {
				writeln!(output, "\tcout << {};", statement[1]).expect("Error writing in file")
			}
			"Fin" => {
				writeln!(output, "}}").expect("Error writing in file");
				break;
			}
			_ => match statement[1] {
				"<-" => {
					write!(output, "\t{} =", statement[0]).expect("Error writing in file");
					for elem in statement.iter().skip(2) {
						write!(output, " {}", elem).expect("Error writing in file");
					}
					writeln!(output, ";").expect("Error writing in file");
				}
				_ => println!("Simbolo no reconocido"),
			},
		}
	}
}

#![feature(io)]
use std::fs::File;
use std::io::BufReader;
// use std::io::BufRead;
use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ID,
    WHILE,
    IF,
    NUM,
    Mas,
    PuntoYComa,
    Mayor,
    MayorIgual,
    Menor,
    MenorIgual,
    Undef
}

use TokenType::*;

fn show_token(token_type: &TokenType, lexema: &String) {
    println!("Token = {:?} [{}]", token_type, lexema);
}

fn get_token_type(c: &str) -> TokenType {
    if c.is_empty() {
        Undef
    } else if c.chars().nth(0).unwrap().is_digit(10) {
        NUM
    } else if c == "while" {
        WHILE
    } else if c == "if" {
        IF
    } else if c == "+" {
        Mas
    } else if c == ";" {
        PuntoYComa
    } else if c == "<" {
        Menor
    } else if c == "<=" {
        MenorIgual
    } else if c == ">" {
        Mayor
    } else if c == ">=" {
        MayorIgual
    } else {
        ID
    }
}

pub fn token_scanner(path: &str) {
    let input = File::open(path).expect("Unable to open");
    let buffer = BufReader::new(input);

    let mut lexema = String::new();
    let mut it = buffer.chars();

    let mut new_token_type = Undef;

    loop {
        if let Some(Ok(c)) = it.next() {
            if !c.is_whitespace() {
                match new_token_type {
                    Undef => {
                        lexema.push(c);
                        new_token_type = get_token_type(&lexema);
                    }
                    NUM => {
                        if !c.is_digit(10) {
                            show_token(&new_token_type, &lexema);
                            lexema.clear();
                            new_token_type = Undef;
                        }
                        lexema.push(c);
                    }
                    Mayor | Menor => {
                        if c == '=' {
                            lexema.push(c);
                            new_token_type = get_token_type(&lexema);
                        }
                        show_token(&new_token_type, &lexema);
                        lexema.clear();
                        lexema.push(c);
                        new_token_type = Undef;
                    }
                    _ => {
                        let prev_token_type = new_token_type;
                        lexema.push(c);
                        new_token_type = get_token_type(&lexema);

                        if prev_token_type != new_token_type {
                            show_token(&new_token_type, &lexema);
                            lexema.clear();
                        }
                    }
                }
            } else {
                if !lexema.is_empty() {
                    show_token(&new_token_type, &lexema);
                    lexema.clear();
                }

                println!("[ ]");
                new_token_type = Undef;
            }
        } else {
            break;
        }
    }

    println!("{}", lexema);
}
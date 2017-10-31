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

    let mut token_type = Undef;

    loop {
        if let Some(Ok(c)) = it.next() {
            if !c.is_whitespace() {
                let mut do_flush = false;

                match token_type {
                    Undef => {}
                    NUM => {
                        if !c.is_digit(10) {
                            do_flush = true;
                        }
                    }
                    ID => {
                        if !c.is_alphanumeric() {
                            do_flush = true;
                        }
                    }
                    Mayor | Menor => {
                        if c != '=' {
                            do_flush = true;
                        }
                    }
                    WHILE | IF => {
                        if c.is_alphanumeric() {
                            token_type = ID;
                        } else {
                            do_flush = true;
                        }
                    }
                    _ => {
                        let mut lexema2 = lexema.clone();
                        lexema2.push(c);
                        let new_token_type = get_token_type(&lexema2);

                        if new_token_type != token_type {
                            do_flush = true;
                        }
                    }
                }

                if do_flush {
                    show_token(&token_type, &lexema);
                    lexema.clear();
                }

                lexema.push(c);
                token_type = get_token_type(&lexema);
            } else {
                if !lexema.is_empty() {
                    show_token(&token_type, &lexema);
                    lexema.clear();
                }
                token_type = Undef;
                println!("-");
            }
        } else {
            break;
        }
    }

    show_token(&token_type, &lexema);
    lexema.clear();
}
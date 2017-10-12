#[derive(PartialEq, Clone, Copy)]
enum State {
    One,
    Two,
    Three,
    Error,
}

use State::*;

fn transition_diagram(input: &str) -> bool {
    let mut state = One;
    for c in input.chars() {
        match state {
            One => if c.is_alphabetic() {
                state = Three;
            } else if c.is_numeric() {
                state = Two;
            } else {
                return false;
            },
            Three => if c.is_alphanumeric() {
                state = Three;
            } else {
                return false;
            },
            _ => return false,
        }
    }

    match state {
        Three => true,
        _ => false,
    }
}

enum Entry {
    Letter,
    Digit,
    // FDC,
}

static TRANS_TBL: [[State; 3]; 3] = [
    [Three, Two, Error],
    [Error, Error, Error],
    // [Three, Three, Accept],
    [Three, Three, Three],
];

use Entry::*;

fn transition_table(input: &str) -> bool {
    let mut state = One;

    for c in input.chars() {
        let entry = if c.is_alphabetic() {
            Letter
        } else if c.is_numeric() {
            Digit
        } else {
            return false;
        };

        state = TRANS_TBL[state as usize][entry as usize];

        if state == Error {
            return false;
        }
    }

    true // state: accept
}

fn main() {
    assert_eq!(true, transition_diagram("aaa"));
    assert_eq!(false, transition_diagram("1a"));
    assert_eq!(false, transition_diagram("^a"));
    assert_eq!(true, transition_diagram("A2B"));
    assert_eq!(false, transition_diagram("~c"));
    assert_eq!(false, transition_diagram("123"));
    assert_eq!(true, transition_diagram("ccc"));

    assert_eq!(true, transition_table("aaa"));
    assert_eq!(false, transition_table("1a"));
    assert_eq!(false, transition_table("^a"));
    assert_eq!(true, transition_table("A2B"));
    assert_eq!(false, transition_table("~c"));
    assert_eq!(false, transition_table("123"));
    assert_eq!(true, transition_table("ccc"));

    println!("Hello, world!");
}

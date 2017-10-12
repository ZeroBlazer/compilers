enum State {
    One,
    Two,
    Three,
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
            Two => return false,
            Three => if c.is_alphanumeric() {
                state = Three;
            } else {
                return false;
            },
        }
    }

    match state {
        Three => true,
        _ => false,
    }
}

fn main() {
    assert_eq!(true, transition_diagram("aaa"));
    assert_eq!(false, transition_diagram("1a"));
    assert_eq!(false, transition_diagram("^a"));
    assert_eq!(true, transition_diagram("A2B"));
    assert_eq!(false, transition_diagram("~c"));
    assert_eq!(false, transition_diagram("123"));
    assert_eq!(true, transition_diagram("ccc"));

    println!("Hello, world!");
}

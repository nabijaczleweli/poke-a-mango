use std::iter::FromIterator;
use poke_a_mango::Error;

mod parse_error;
mod io;


#[test]
fn ui() {
    let mut out = Vec::new();
    Error::Ui {
            desc: "create window",
            error: "Burgeoisie not abolished".to_string(),
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
               "Failed to create window: Burgeoisie not abolished.\n".to_string());
}

extern crate poke_a_mango;

use self::poke_a_mango::Error;
use std::iter::FromIterator;


#[test]
fn normal_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "leaderboard",
            op: "print",
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Printing leaderboard failed.\n".to_string());
}

#[test]
fn normal_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "leaderboard",
            op: "write",
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Writing leaderboard failed.\n".to_string());
}

#[test]
fn single_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "leaderboard",
            op: "t",
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Ting leaderboard failed.\n".to_string());
}

#[test]
fn single_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "leaderboard",
            op: "e",
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "ing leaderboard failed.\n".to_string());
}

#[test]
fn empty() {
    let mut out = Vec::new();
    Error::Io {
            desc: "leaderboard",
            op: "",
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "ing leaderboard failed.\n".to_string());
}

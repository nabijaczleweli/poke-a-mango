use std::iter::FromIterator;
use poke_a_mango::Error;


#[test]
fn empty() {
    let mut out = Vec::new();
    Error::FileParsingFailed {
            desc: "leaderboard",
            errors: vec![],
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)), "Failed to parse leaderboard.\n".to_string());
}

#[test]
fn some() {
    let mut out = Vec::new();
    Error::FileParsingFailed {
            desc: "leaderboard",
            errors: vec!["lbrd.toml:10: invalid value".to_string(), "lbrd.toml:15: capitalism overload".to_string()],
        }
        .print_error(&mut out);
    assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
               "Failed to parse leaderboard:\n\x20\x20lbrd.toml:10: invalid value\n\x20\x20lbrd.toml:15: capitalism overload\n".to_string());
}

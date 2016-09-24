use poke_a_mango::util::uppercase_first;


#[test]
fn simple() {
    assert_eq!(uppercase_first("alfa"), "Alfa".to_string());
    assert_eq!(uppercase_first("delta"), "Delta".to_string());
    assert_eq!(uppercase_first("golf"), "Golf".to_string());
    assert_eq!(uppercase_first("juliett"), "Juliett".to_string());
    assert_eq!(uppercase_first("mike"), "Mike".to_string());
    assert_eq!(uppercase_first("papa"), "Papa".to_string());
    assert_eq!(uppercase_first("whiskey"), "Whiskey".to_string());
}

#[test]
fn already_upper() {
    assert_eq!(uppercase_first("Tango"), "Tango".to_string());
    assert_eq!(uppercase_first("Zulu"), "Zulu".to_string());
    assert_eq!(uppercase_first("India"), "India".to_string());
    assert_eq!(uppercase_first("Sierra"), "Sierra".to_string());
}

#[test]
fn single() {
    assert_eq!(uppercase_first("c"), "C".to_string());
    assert_eq!(uppercase_first("u"), "U".to_string());
}

#[test]
fn single_already_upper() {
    assert_eq!(uppercase_first("R"), "R".to_string());
    assert_eq!(uppercase_first("N"), "N".to_string());
}

#[test]
fn multi() {
    assert_eq!(uppercase_first("bravo foxtrot X-ray yankee."), "Bravo foxtrot X-ray yankee.".to_string());
}

#[test]
fn multi_already_upper() {
    assert_eq!(uppercase_first("Echo kilo hotel Victor."), "Echo kilo hotel Victor.".to_string());
}

#[test]
fn empty() {
    assert!(uppercase_first("").is_empty());
}

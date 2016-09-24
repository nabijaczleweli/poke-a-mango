use poke_a_mango::Error;


#[test]
fn parse_error() {
    assert_eq!(Error::FileParsingFailed {
                       desc: "",
                       errors: vec![],
                   }
                   .exit_value(),
               1);
}

#[test]
fn io() {
    assert_eq!(Error::Io { desc: "", op: "" }.exit_value(), 2);
}

use poke_a_mango::ops::Leader;
use self::super::make_dir;
use poke_a_mango::Error;
use chrono::DateTime;
use std::io::Write;
use std::fs::File;


#[test]
fn nonexistant() {
    let tf = make_dir("read", "nonexistant");
    assert_eq!(Leader::read(&tf), Ok(vec![]));
}

#[test]
fn empty() {
    let tf = make_dir("read", "empty");
    File::create(&tf).unwrap().write_all(b"leader = []\n").unwrap();

    assert_eq!(Leader::read(&tf), Ok(vec![]));
}

#[test]
fn single() {
    let tf = make_dir("read", "single");
    File::create(&tf)
        .unwrap()
        .write_all(b"[[leader]]\n\
               name = \"nabijaczleweli\"\n\
               score = 101\n\
               time = \"1970-01-01T00:00:00+00:00\"\n")
        .unwrap();

    assert_eq!(Leader::read(&tf),
               Ok(vec![Leader {
                           name: "nabijaczleweli".to_string(),
                           time: DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 GMT").unwrap(),
                           score: 101,
                       }]));
}

#[test]
fn multi() {
    let tf = make_dir("read", "multi");
    File::create(&tf)
        .unwrap()
        .write_all(b"[[leader]]\n\
               name = \"nabijaczleweli\"\n\
               score = 101\n\
               time = \"1970-01-01T00:00:00+00:00\"\n\
               \n\
               [[leader]]\n\
               name = \"skorezore\"\n\
               score = 50\n\
               time = \"1971-01-01T01:00:00+01:00\"\n\
               \n\
               [[leader]]\n\
               name = \"sehe\"\n\
               score = 4\n\
               time = \"1971-01-01T09:00:00-01:00\"\n")
        .unwrap();


    assert_eq!(Leader::read(&tf),
               Ok(vec![Leader {
                           name: "nabijaczleweli".to_string(),
                           time: DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 GMT").unwrap(),
                           score: 101,
                       },
                       Leader {
                           name: "skorezore".to_string(),
                           time: DateTime::parse_from_rfc2822("Fri, 01 Jan 1971 01:00:00 +0100").unwrap(),
                           score: 50,
                       },
                       Leader {
                           name: "sehe".to_string(),
                           time: DateTime::parse_from_rfc2822("Fri, 01 Jan 1971 09:00:00 -0100").unwrap(),
                           score: 4,
                       }]));
}

#[test]
fn invalid() {
    let tf = make_dir("read", "invalid");
    File::create(&tf).unwrap().write_all(b"leader = [\n").unwrap();

    assert_eq!(Leader::read(&tf),
               Err(Error::FileParsingFailed {
                   desc: "leaderboard",
                   errors: vec!["error: 1:0: expected a value".to_string()],
               }));
}

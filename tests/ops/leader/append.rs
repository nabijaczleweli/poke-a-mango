use poke_a_mango::ops::Leader;
use std::io::{Read, Write};
use self::super::make_dir;
use std::fs::{self, File};
use chrono::DateTime;


#[test]
fn empty() {
    let tf = make_dir("append", "empty");
    let _ = fs::remove_file(&tf);
    assert_eq!(Leader::append(Leader {
                                  name: "nabijaczleweli".to_string(),
                                  time: DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 GMT").unwrap(),
                                  score: 101,
                              },
                              &tf),
               Ok(()));

    let mut buf = String::new();
    File::open(&tf).unwrap().read_to_string(&mut buf).unwrap();
    assert_eq!(buf,
               "[[leader]]\n\
               name = \"nabijaczleweli\"\n\
               score = 101\n\
               time = \"1970-01-01T00:00:00+00:00\"\n"
                   .to_string());
}

#[test]
fn multi() {
    let tf = make_dir("append", "multi");
    File::create(&tf)
        .unwrap()
        .write_all(b"[[leader]]\n\
               name = \"nabijaczleweli\"\n\
               score = 200\n\
               time = \"1970-01-01T00:00:00+00:00\"\n\
               \n\
               [[leader]]\n\
               name = \"sehe\"\n\
               score = 50\n\
               time = \"1970-01-01T00:00:00+00:00\"\n")
        .unwrap();
    assert_eq!(Leader::append(Leader {
                                  name: "nabijaczleweli".to_string(),
                                  time: DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 GMT").unwrap(),
                                  score: 101,
                              },
                              &tf),
               Ok(()));

    let mut buf = String::new();
    File::open(&tf).unwrap().read_to_string(&mut buf).unwrap();
    assert_eq!(buf,
               "[[leader]]\n\
               name = \"nabijaczleweli\"\n\
               score = 200\n\
               time = \"1970-01-01T00:00:00+00:00\"\n\
               \n\
               [[leader]]\n\
               name = \"nabijaczleweli\"\n\
               score = 101\n\
               time = \"1970-01-01T00:00:00+00:00\"\n\
               \n\
               [[leader]]\n\
               name = \"sehe\"\n\
               score = 50\n\
               time = \"1970-01-01T00:00:00+00:00\"\n"
                   .to_string());
}

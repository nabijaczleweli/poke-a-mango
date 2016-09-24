use poke_a_mango::ops::Leader;
use self::super::make_dir;
use std::fs::{self, File};
use poke_a_mango::Error;
use chrono::DateTime;
use std::io::Read;


#[test]
fn empty() {
    let tf = make_dir("write", "empty");
    assert_eq!(Leader::write(vec![], &tf), Ok(()));

    let mut buf = String::new();
    File::open(&tf).unwrap().read_to_string(&mut buf).unwrap();
    assert_eq!(buf, "leader = []\n".to_string());
}

#[test]
fn single() {
    let tf = make_dir("write", "single");
    assert_eq!(Leader::write(vec![Leader {
                                      name: "nabijaczleweli".to_string(),
                                      time: DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 GMT").unwrap(),
                                      score: 101,
                                  }],
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
    let tf = make_dir("write", "multi");
    assert_eq!(Leader::write(vec![Leader {
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
                                  }],
                             &tf),
               Ok(()));

    let mut buf = String::new();
    File::open(&tf).unwrap().read_to_string(&mut buf).unwrap();
    assert_eq!(buf,
               "[[leader]]\n\
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
               time = \"1971-01-01T09:00:00-01:00\"\n"
                   .to_string());
}

#[test]
fn locked() {
    let tf = make_dir("write", "locked");

    // Only create and set R/O if the file doesn't exist already
    if let Ok(lock) = File::create(&tf) {
        let mut lock_perms = lock.metadata().unwrap().permissions();
        lock_perms.set_readonly(true);
        fs::set_permissions(&tf, lock_perms.clone()).unwrap();
    }

    assert_eq!(Leader::write(vec![], &tf),
               Err(Error::Io {
                   desc: "leaderboard",
                   op: "create",
               }));
}

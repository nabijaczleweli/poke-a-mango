use poke_a_mango::ops::Leader;
use std::fs::create_dir;
use std::path::PathBuf;
use std::env::temp_dir;
use chrono::Local;

mod append;
mod write;
mod read;


#[test]
fn now() {
    let before = Local::now();
    let before = before.with_timezone(before.offset());
    let ldr = Leader::now("nabijaczleweli".to_string(), 36);
    let after = Local::now();
    let after = after.with_timezone(after.offset());

    assert_eq!(ldr.name, "nabijaczleweli");
    assert!(ldr.time >= before);
    assert!(ldr.time <= after);
    assert_eq!(ldr.score, 36);
}


fn make_dir(section: &str, function: &str) -> PathBuf {
    let mut tf = temp_dir();
    let _ = create_dir(&tf);
    tf.push("poke-a-mango-test");
    let _ = create_dir(&tf);
    tf.push(format!("leader-{}-{}", section, function));
    let _ = create_dir(&tf);
    tf.join("leaderboard.toml")
}

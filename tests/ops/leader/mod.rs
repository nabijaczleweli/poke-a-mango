use std::fs::create_dir;
use std::path::PathBuf;
use std::env::temp_dir;

mod write;
mod read;


fn make_dir(section: &str, function: &str) -> PathBuf {
    let mut tf = temp_dir();
    let _ = create_dir(&tf);
    tf.push("poke-a-mango-test");
    let _ = create_dir(&tf);
    tf.push(format!("leader-{}-{}", section, function));
    let _ = create_dir(&tf);
    tf.join("leaderboard.toml")
}

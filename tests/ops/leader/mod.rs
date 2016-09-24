use std::fs::create_dir_all;
use std::path::PathBuf;
use std::env::temp_dir;

mod write;
mod read;


fn make_dir(section: &str, function: &str) -> PathBuf {
    let tf = temp_dir().join("poke-a-mango-test").join(format!("leader-{}-{}", section, function));
    create_dir_all(&tf).unwrap();
    tf.join("leaderboard.toml")
}

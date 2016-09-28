use poke_a_mango::ops::{Difficulty, GameState};


#[test]
fn should_exit() {
    assert_eq!(GameState::MainMenu.should_exit(), false);
    assert_eq!(GameState::ChooseDifficulty.should_exit(), false);
    assert_eq!(GameState::Playing(Difficulty::Easy).should_exit(), false);
    assert_eq!(GameState::Playing(Difficulty::Normal).should_exit(), false);
    assert_eq!(GameState::Playing(Difficulty::Hard).should_exit(), false);
    assert_eq!(GameState::DisplayLeaderboard.should_exit(), false);
    assert_eq!(GameState::Exit.should_exit(), true);
}

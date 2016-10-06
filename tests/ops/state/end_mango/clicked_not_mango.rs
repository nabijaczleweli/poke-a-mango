use poke_a_mango::ops::{Difficulty, GameState, Leader};
use poke_a_mango::ops::state;


#[test]
fn from_main_menu() {
    let mut s = GameState::MainMenu;
    state::end_mango(&mut s, true, false);
    assert_eq!(s, GameState::MainMenu);
}

#[test]
fn from_choose_difficulty() {
    let mut s = GameState::ChooseDifficulty;
    state::end_mango(&mut s, true, false);
    assert_eq!(s, GameState::ChooseDifficulty);
}

#[test]
fn from_playing() {
    let state = GameState::Playing {
        difficulty: Difficulty::Easy,
        score: 101,
        fruit: None,
    };

    let mut s = state.clone();
    state::end_mango(&mut s, true, false);
    assert_eq!(s,
               GameState::GameOver {
                   difficulty: Difficulty::Easy,
                   score: 101,
                   name: "Your name".to_string(),
               });
}

#[test]
fn from_choose_game_ended() {
    let state = GameState::GameEnded {
        score: 25,
        name: "наб".to_string(),
    };

    let mut s = state.clone();
    state::end_mango(&mut s, true, false);
    assert_eq!(s, state);
}

#[test]
fn from_choose_load_leaderboard() {
    let mut s = GameState::LoadLeaderboard;
    state::end_mango(&mut s, true, false);
    assert_eq!(s, GameState::LoadLeaderboard);
}

#[test]
fn from_choose_display_leaderboard() {
    let state = GameState::DisplayLeaderboard(vec![Leader::now("наб".to_string(), 25)]);

    let mut s = state.clone();
    state::end_mango(&mut s, true, false);
    assert_eq!(s, state);
}

#[test]
fn from_choose_exit() {
    let mut s = GameState::Exit;
    state::end_mango(&mut s, true, false);
    assert_eq!(s, GameState::Exit);
}

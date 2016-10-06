use poke_a_mango::ops::{Difficulty, GameState, Leader};
use poke_a_mango::ops::state;


#[test]
fn from_main_menu() {
    let mut s = GameState::MainMenu;
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, GameState::MainMenu);
}

#[test]
fn from_choose_difficulty_easy() {
    let mut s = GameState::ChooseDifficulty;
    state::select_difficulty(&mut s, Difficulty::Easy);
    assert_eq!(s,
               GameState::Playing {
                   difficulty: Difficulty::Easy,
                   score: 0,
                   fruit: None,
               });
}

#[test]
fn from_choose_difficulty_normal() {
    let mut s = GameState::ChooseDifficulty;
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s,
               GameState::Playing {
                   difficulty: Difficulty::Normal,
                   score: 0,
                   fruit: None,
               });
}

#[test]
fn from_choose_difficulty_hard() {
    let mut s = GameState::ChooseDifficulty;
    state::select_difficulty(&mut s, Difficulty::Hard);
    assert_eq!(s,
               GameState::Playing {
                   difficulty: Difficulty::Hard,
                   score: 0,
                   fruit: None,
               });
}

#[test]
fn from_playing() {
    let state = GameState::Playing {
        difficulty: Difficulty::Easy,
        score: 101,
        fruit: None,
    };

    let mut s = state.clone();
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, state);
}

#[test]
fn from_choose_game_over() {
    let state = GameState::GameOver {
        difficulty: Difficulty::Easy,
        score: 51,
        name: "наб".to_string(),
    };

    let mut s = state.clone();
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, state);
}

#[test]
fn from_choose_game_ended() {
    let state = GameState::GameEnded {
        score: 25,
        name: "наб".to_string(),
    };

    let mut s = state.clone();
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, state);
}

#[test]
fn from_choose_load_leaderboard() {
    let mut s = GameState::LoadLeaderboard;
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, GameState::LoadLeaderboard);
}

#[test]
fn from_choose_display_leaderboard() {
    let state = GameState::DisplayLeaderboard(vec![Leader::now("наб".to_string(), 25)]);

    let mut s = state.clone();
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, state);
}

#[test]
fn from_choose_exit() {
    let mut s = GameState::Exit;
    state::select_difficulty(&mut s, Difficulty::Normal);
    assert_eq!(s, GameState::Exit);
}

use poke_a_mango::ops::{Difficulty, GameState, Leader};
use poke_a_mango::ops::state;


#[test]
fn from_main_menu() {
    let mut s = GameState::MainMenu;
    state::submit_name(&mut s);
    assert_eq!(s, GameState::MainMenu);
}

#[test]
fn from_choose_difficulty() {
    let mut s = GameState::ChooseDifficulty;
    state::submit_name(&mut s);
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
    state::submit_name(&mut s);
    assert_eq!(s, state);
}

#[test]
fn from_choose_game_over_with_difficulty_easy() {
    let mut s = GameState::GameOver {
        difficulty: Difficulty::Easy,
        score: 51,
        name: "наб".to_string(),
    };
    state::submit_name(&mut s);
    assert_eq!(s,
               GameState::GameEnded {
                   name: "наб".to_string(),
                   score: (51f64 * Difficulty::Easy.point_weight()).floor() as u64,
               });
}

#[test]
fn from_choose_game_over_with_difficulty_normal() {
    let mut s = GameState::GameOver {
        difficulty: Difficulty::Normal,
        score: 51,
        name: "наб".to_string(),
    };
    state::submit_name(&mut s);
    assert_eq!(s,
               GameState::GameEnded {
                   name: "наб".to_string(),
                   score: (51f64 * Difficulty::Normal.point_weight()).floor() as u64,
               });
}

#[test]
fn from_choose_game_over_with_difficulty_hard() {
    let mut s = GameState::GameOver {
        difficulty: Difficulty::Hard,
        score: 51,
        name: "наб".to_string(),
    };
    state::submit_name(&mut s);
    assert_eq!(s,
               GameState::GameEnded {
                   name: "наб".to_string(),
                   score: (51f64 * Difficulty::Hard.point_weight()).floor() as u64,
               });
}

#[test]
fn from_choose_game_ended() {
    let state = GameState::GameEnded {
        score: 25,
        name: "наб".to_string(),
    };

    let mut s = state.clone();
    state::submit_name(&mut s);
    assert_eq!(s, state);
}

#[test]
fn from_choose_load_leaderboard() {
    let mut s = GameState::LoadLeaderboard;
    state::submit_name(&mut s);
    assert_eq!(s, GameState::LoadLeaderboard);
}

#[test]
fn from_choose_display_leaderboard() {
    let state = GameState::DisplayLeaderboard(vec![Leader::now("наб".to_string(), 25)]);

    let mut s = state.clone();
    state::submit_name(&mut s);
    assert_eq!(s, state);
}

#[test]
fn from_choose_exit() {
    let mut s = GameState::Exit;
    state::submit_name(&mut s);
    assert_eq!(s, GameState::Exit);
}

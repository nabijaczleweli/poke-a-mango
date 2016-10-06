use poke_a_mango::ops::{Difficulty, GameState, Leader};
use poke_a_mango::ops::state;


#[test]
fn from_main_menu() {
    let mut s = GameState::MainMenu;
    state::tick_mango(&mut s);
    assert_eq!(s, GameState::MainMenu);
}

#[test]
fn from_choose_difficulty() {
    let mut s = GameState::ChooseDifficulty;
    state::tick_mango(&mut s);
    assert_eq!(s, GameState::ChooseDifficulty);
}

#[test]
fn from_playing_mango() {
    let mut s = GameState::Playing {
        difficulty: Difficulty::Easy,
        score: 101,
        fruit: None,
    };
    assert_eq!(state::tick_mango(&mut s), None);

    match s {
        GameState::Playing { difficulty, score, fruit: _ } => {
            assert_eq!(score, 101);
            assert_eq!(difficulty, Difficulty::Easy);
        }
        _ => assert!(false),
    }
}

#[test]
fn from_playing_not_mango() {
    let mut s = GameState::Playing {
        difficulty: Difficulty::Easy,
        score: 101,
        fruit: Some(0),
    };
    assert_eq!(state::tick_mango(&mut s), Some(0));

    match s {
        GameState::Playing { difficulty, score, fruit: _ } => {
            assert_eq!(score, 101);
            assert_eq!(difficulty, Difficulty::Easy);
        }
        _ => assert!(false),
    }
}

#[test]
fn from_choose_game_over() {
    let state = GameState::GameOver {
        difficulty: Difficulty::Easy,
        score: 51,
        name: "наб".to_string(),
    };

    let mut s = state.clone();
    state::tick_mango(&mut s);
    assert_eq!(s, state);
}

#[test]
fn from_choose_game_ended() {
    let state = GameState::GameEnded {
        score: 25,
        name: "наб".to_string(),
    };

    let mut s = state.clone();
    state::tick_mango(&mut s);
    assert_eq!(s, state);
}

#[test]
fn from_choose_load_leaderboard() {
    let mut s = GameState::LoadLeaderboard;
    state::tick_mango(&mut s);
    assert_eq!(s, GameState::LoadLeaderboard);
}

#[test]
fn from_choose_display_leaderboard() {
    let state = GameState::DisplayLeaderboard(vec![Leader::now("наб".to_string(), 25)]);

    let mut s = state.clone();
    state::tick_mango(&mut s);
    assert_eq!(s, state);
}

#[test]
fn from_choose_exit() {
    let mut s = GameState::Exit;
    state::tick_mango(&mut s);
    assert_eq!(s, GameState::Exit);
}

use poke_a_mango::ops::{Difficulty, GameState};


#[test]
fn should_exit() {
    assert_eq!(GameState::MainMenu.should_exit(), false);
    assert_eq!(GameState::ChooseDifficulty.should_exit(), false);
    assert_eq!(GameState::Playing {
                       difficulty: Difficulty::Easy,
                       score: 0,
                       mango: false,
                   }
                   .should_exit(),
               false);
    assert_eq!(GameState::Playing {
                       difficulty: Difficulty::Normal,
                       score: 0,
                       mango: false,
                   }
                   .should_exit(),
               false);
    assert_eq!(GameState::Playing {
                       difficulty: Difficulty::Hard,
                       score: 0,
                       mango: false,
                   }
                   .should_exit(),
               false);
    assert_eq!(GameState::LoadLeaderboard.should_exit(), false);
    assert_eq!(GameState::DisplayLeaderboard(vec![]).should_exit(), false);
    assert_eq!(GameState::Exit.should_exit(), true);
}

#[test]
fn should_load_leaderboard() {
    assert_eq!(GameState::MainMenu.should_load_leaderboard(), false);
    assert_eq!(GameState::ChooseDifficulty.should_load_leaderboard(), false);
    assert_eq!(GameState::Playing {
                       difficulty: Difficulty::Easy,
                       score: 0,
                       mango: false,
                   }
                   .should_load_leaderboard(),
               false);
    assert_eq!(GameState::Playing {
                       difficulty: Difficulty::Normal,
                       score: 0,
                       mango: false,
                   }
                   .should_load_leaderboard(),
               false);
    assert_eq!(GameState::Playing {
                       difficulty: Difficulty::Hard,
                       score: 0,
                       mango: false,
                   }
                   .should_load_leaderboard(),
               false);
    assert_eq!(GameState::LoadLeaderboard.should_load_leaderboard(), true);
    assert_eq!(GameState::DisplayLeaderboard(vec![]).should_load_leaderboard(), false);
    assert_eq!(GameState::Exit.should_load_leaderboard(), false);
}

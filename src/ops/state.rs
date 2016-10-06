//! Manual `GameState` manipulation, to be used when implementing a UI.
//!
//! For a UI implementing the use of this see the implementation of `Widgets::update()`.


use self::super::{Difficulty, GameState};
use self::super::super::util::FRUITS;
use rand::{Rng, thread_rng};


/// Press the Start button in MainMenu, transforming it into ChooseDifficulty.
///
/// If the supplied argument is not MainMenu, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, GameState};
/// # let start_button_pressed = true;
/// let mut state = GameState::MainMenu;
/// if start_button_pressed {
///     state::press_start(&mut state);
/// }
/// ```
pub fn press_start(s: &mut GameState) {
    match s {
        &mut GameState::MainMenu => *s = GameState::ChooseDifficulty,
        _ => {}
    }
}

/// Press the Display Highscores button in MainMenu, transforming it into LoadLeaderboard.
///
/// If the supplied argument is not MainMenu, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, GameState};
/// # let display_highscores_button_pressed = true;
/// let mut state = GameState::MainMenu;
/// if display_highscores_button_pressed {
///     state::press_display_highscores(&mut state);
///     // Then probably load the leaderboard
/// }
/// ```
pub fn press_display_highscores(s: &mut GameState) {
    match s {
        &mut GameState::MainMenu => *s = GameState::LoadLeaderboard,
        _ => {}
    }
}

/// Press the Exit button in MainMenu, transforming it into Exit.
///
/// If the supplied argument is not MainMenu, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, GameState};
/// # let exit_button_pressed = true;
/// let mut state = GameState::MainMenu;
/// if exit_button_pressed {
///     state::press_exit(&mut state);
///     // Then probably close the window
/// }
/// ```
pub fn press_exit(s: &mut GameState) {
    match s {
        &mut GameState::MainMenu => *s = GameState::Exit,
        _ => {}
    }
}

/// Press one of the Difficulty buttons in ChooseDifficulty, transforming it into Exit.
///
/// If the supplied argument is not ChooseDifficulty, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, Difficulty, GameState};
/// # let difficulty_easy_button_pressed = false;
/// # let difficulty_normal_button_pressed = true;
/// # let difficulty_hard_button_pressed = false;
/// let mut state = GameState::ChooseDifficulty;
/// if difficulty_easy_button_pressed {
///     state::select_difficulty(&mut state, Difficulty::Easy);
/// } else if difficulty_normal_button_pressed {
///     state::select_difficulty(&mut state, Difficulty::Normal);
/// } else if difficulty_hard_button_pressed {
///     state::select_difficulty(&mut state, Difficulty::Hard);
/// }
/// ```
pub fn select_difficulty(s: &mut GameState, difficulty: Difficulty) {
    match s {
        &mut GameState::ChooseDifficulty => {
            *s = GameState::Playing {
                difficulty: difficulty,
                score: 0,
                fruit: None,
            }
        }
        _ => {}
    }
}

/// Press the Back button in ChooseDifficulty, GameOver or DisplayLeaderboard, transforming them into MainMenu.
///
/// If the supplied argument is not ChooseDifficulty, GameOver or DisplayLeaderboard, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, GameState};
/// # let back_button_pressed = false;
/// let mut state = GameState::ChooseDifficulty;
/// if back_button_pressed {
///     state::press_back(&mut state);
/// }
/// ```
pub fn press_back(s: &mut GameState) {
    match s {
        &mut GameState::ChooseDifficulty |
        &mut GameState::GameOver { difficulty: _, score: _, name: _ } |
        &mut GameState::DisplayLeaderboard(_) => *s = GameState::MainMenu,
        _ => {}
    }
}

/// Tick the mango in Playing state.
///
/// Call this before displaying/updating the label of the mango button. Returns the original fruit.
///
/// If the supplied argument is not Playing, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, Difficulty, GameState};
/// # use poke_a_mango::util::fruit_name;
/// # let mango_button = ();
/// # fn update_label(_: &(), _: &'static str) {}
/// let mut state = GameState::Playing {
///     difficulty: Difficulty::Hard,
///     score: 12,
///     fruit: None,
/// };
/// let fruit = state::tick_mango(&mut state);
/// update_label(&mango_button, fruit_name(&fruit));
/// ```
pub fn tick_mango(s: &mut GameState) -> Option<usize> {
    match s {
        &mut GameState::Playing { difficulty, score: _, ref mut fruit } => {
            let original_fruit = *fruit;

            // Difficulty's numeric value is inverted here
            let mut rng = thread_rng();
            let change_fruit = rng.gen_weighted_bool(25 * (4 - difficulty.numeric()));
            if change_fruit {
                *fruit = if fruit.is_none() {
                    Some(rng.gen_range(0, FRUITS.len()))
                } else {
                    None
                };
            }

            original_fruit
        }
        _ => None,
    }
}

/// End the mango processing sequence, given whether it was clicked and whether it was actually a mango.
///
/// If the supplied argument is not Playing or the button wasn't clicked, it remains unchanged.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::ops::{state, Difficulty, GameState};
/// # use poke_a_mango::util::fruit_name;
/// # let was_mango = true;
/// # let mango_button_clicked = true;
/// let mut state = GameState::Playing {
///     difficulty: Difficulty::Hard,
///     score: 12,
///     fruit: None,
/// };
/// state::end_mango(&mut state, mango_button_clicked, was_mango);
/// ```
pub fn end_mango(s: &mut GameState, clicked: bool, was_mango: bool) {
    if clicked {
        if was_mango {
            match s {
                &mut GameState::Playing { difficulty: _, ref mut score, fruit: _ } => *score += 1,
                _ => {}
            }
        } else {
            match s {
                &mut GameState::Playing { difficulty, score, fruit: _ } => {
                    *s = GameState::GameOver {
                        difficulty: difficulty,
                        score: score,
                        name: "Your name".to_string(),
                    };
                }
                _ => {}
            }
        }
    }
}

pub fn submit_name(s: &mut GameState) {
    match s {
        &mut GameState::GameOver { difficulty, score, name: _ } => {
            let name = match s {
                &mut GameState::GameOver { difficulty: _, score: _, ref name } => name.clone(),
                _ => "".to_string(),
            };
            *s = GameState::GameEnded {
                name: name,
                score: ((score as f64) * difficulty.point_weight()).floor() as u64,
            };
        }
        _ => {}
    }
}

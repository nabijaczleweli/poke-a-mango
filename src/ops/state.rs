use self::super::Leader;


/// Game's all possible states.
///
/// `Widgets::update()` takes care of proper state transformation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    /// Display the main menu.
    ///
    /// Main menu contains a Start button and an Exit button.
    ///
    /// Can transform into:
    ///
    ///   * `ChooseDifficulty`
    ///   * `DisplayHighscores`
    ///   * `Exit`
    MainMenu,
    /// Display the screen where the player chooses the difficulty.
    ///
    /// The screen contains one button for each difficulty and the Back button.
    ///
    /// Can transform into:
    ///
    ///   * `Playing`
    ///   * `MainMenu`
    ChooseDifficulty,
    /// The game is currently in progress.
    ///
    /// This also contains the game's difficulty.
    ///
    /// Can transform into:
    ///
    ///   * `MainMenu`
    ///   * `Exit`
    Playing(Difficulty),
    /// Meta-state indicating that the leaderboard needs to be loaded.
    ///
    /// Needs to be handled in usercode, place the leaderboard into `DisplayLeaderboard` after loading it.
    ///
    /// Leaderboards are loaded via `Leader::load()`.
    ///
    /// Transforms into `DisplayLeaderboard`
    LoadLeaderboard,
    /// Display top 10 high scores.
    ///
    /// This screen also contains the Back button.
    ///
    /// Can transform into:
    ///
    ///   * `MainMenu`
    ///   * `Exit`
    DisplayLeaderboard(Vec<Leader>),
    /// Pseudo-state, signifying that the game window should be closed.
    ///
    /// Can transform into: nothing. This is the final state all others seek.
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl GameState {
    /// Check whether this state currently means that the game has ended.
    ///
    /// # Examples
    ///
    /// ```
    /// # use poke_a_mango::ops::GameState;
    /// assert!(GameState::Exit.should_exit());
    /// ```
    pub fn should_exit(&self) -> bool {
        *self == GameState::Exit
    }

    /// Check whether this state requires usercode to load the leaderboard.
    ///
    /// # Examples
    ///
    /// ```
    /// # use poke_a_mango::ops::GameState;
    /// assert!(GameState::LoadLeaderboard.should_load_leaderboard());
    /// ```
    pub fn should_load_leaderboard(&self) -> bool {
        *self == GameState::LoadLeaderboard
    }
}

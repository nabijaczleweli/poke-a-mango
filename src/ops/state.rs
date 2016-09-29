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
    /// This also contains the game's difficulty, the player's current score and whether the current fruit is a mango.
    ///
    /// Can transform into `GameOver`.
    Playing {
        /// The game difficulty, as chosen in the `ChooseDifficulty` stage.
        difficulty: Difficulty,
        /// The user's current score.
        score: u64,
        /// Whether the current fruit is a mango.
        mango: bool,
    },
    /// The game was lost after a valiant battle.
    ///
    /// Contains the game's difficulty and the players final score.
    ///
    /// In this stage the player enters its name.
    ///
    /// Can transform into `GameEnded`.
    GameOver {
        /// The game difficulty, as same as in the `Playing` stage.
        difficulty: Difficulty,
        /// The user's final score.
        score: u64,
        /// The user's name, mostly partial.
        name: String,
    },
    /// The game cycle has ended. Semi-meta-state
    ///
    /// Contains the game's difficulty and the players final score.
    ///
    /// In this stage the player enters its name.
    ///
    /// Transforms into `MainMenu`.
    GameEnded {
        /// The user's name.
        name: String,
        /// User's final score, weighted.
        score: u64,
    },
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

/// The game's difficulty, chosen in the `ChooseDifficulty` step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    /// Easy difficulty, points are halved when sorting.
    Easy,
    /// Normal difficulty, points are untouched when sorting.
    Normal,
    /// Hard difficulty, points are doubled when sorting.
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

impl Difficulty {
    /// The difficulty's numeric value, from `1` to `3`.
    pub fn numeric(&self) -> u32 {
        match *self {
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }

    /// How to much to multiply the player's points for sorting.
    ///
    /// Points on Easy are worth half, and on Hard - twice the normal amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use poke_a_mango::ops::Difficulty;
    /// // Normally you'd get these by playing
    /// let points = 20;
    /// let difficulty = Difficulty::Hard;
    ///
    /// let real_points = (points as f64) * difficulty.point_weight();
    /// assert_eq!(real_points, 40.0);
    /// ```
    pub fn point_weight(&self) -> f64 {
        match *self {
            Difficulty::Easy => 0.5,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 2.0,
        }
    }
}

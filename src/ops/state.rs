/// All game's possible states.
///
/// `Widgets::update()` takes care of proper state transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Can transform into:
    ///
    ///   * `MainMenu`
    ///   * `Exit`
    Playing,
    /// Display top 10 high scores.
    ///
    /// This screen also contains the Back button.
    ///
    /// Can transform into:
    ///
    ///   * `MainMenu`
    ///   * `Exit`
    DisplayLeaderboard,
    /// Pseudo-state, signifying that the game window should be closed.
    ///
    /// Can transform into: nothing. This is the final state all others seek.
    Exit,
}

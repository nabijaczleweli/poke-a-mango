use self::super::{Difficulty, GameState, set_button_style};
use conrod::{Colorable, Labelable, Widget, UiCell};
use conrod::widget::button::{Button, Flat};
use conrod::widget::id::{Generator, Id};
use conrod::{Positionable, Sizeable};
use conrod::color::DARK_CHARCOAL;
use conrod::widget::Canvas;


/// Container for all widgets' IDs, also manages setting them.
///
/// The general idea is to call `Widgets::new()` once and then `Widgets::update()` each update event.
///
/// An external (usercode) variable is recommended to track the game's state.
///
/// # Examples
///
/// ```
/// # extern crate poke_a_mango;
/// # extern crate conrod;
/// # use poke_a_mango::ops::{GameState, Widgets};
/// # struct Event;
/// # impl Event {
/// #     pub fn update<F: FnOnce(())>(&self, f: F) {
/// #         f(());
/// #     }
/// # }
/// # fn main() {
/// let mut ui = conrod::UiBuilder::new().build();
/// let widgets = Widgets::new(ui.widget_id_generator());
///
/// let mut game_state = GameState::MainMenu;
///
/// // Then, in the event loop
/// # let event = Event;
/// event.update(|_| {
///     game_state = widgets.update(ui.set_widgets(), game_state)
/// });
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Widgets {
    main_canvas: Id,

    start_button_canvas: Id,
    leaderboard_button_canvas: Id,
    exit_button_canvas: Id,

    start_button: Id,
    leaderboard_button: Id,
    exit_button: Id,

    easy_button_canvas: Id,
    normal_button_canvas: Id,
    hard_button_canvas: Id,
    back_button_canvas: Id,

    easy_button: Id,
    normal_button: Id,
    hard_button: Id,
    back_button: Id,
}

impl Widgets {
    /// Generate widget IDs in preparation for later setting them in `Ui`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate poke_a_mango;
    /// # extern crate conrod;
    /// # use poke_a_mango::ops::Widgets;
    /// # fn main() {
    /// let mut ui = conrod::UiBuilder::new().build();
    /// let widgets = Widgets::new(ui.widget_id_generator());
    /// # }
    /// ```
    pub fn new(mut id_gen: Generator) -> Widgets {
        Widgets {
            main_canvas: id_gen.next(),
            start_button_canvas: id_gen.next(),
            leaderboard_button_canvas: id_gen.next(),
            exit_button_canvas: id_gen.next(),
            start_button: id_gen.next(),
            leaderboard_button: id_gen.next(),
            exit_button: id_gen.next(),
            easy_button_canvas: id_gen.next(),
            normal_button_canvas: id_gen.next(),
            hard_button_canvas: id_gen.next(),
            back_button_canvas: id_gen.next(),
            easy_button: id_gen.next(),
            normal_button: id_gen.next(),
            hard_button: id_gen.next(),
            back_button: id_gen.next(),
        }
    }

    /// Update the UI elements and set them.
    ///
    /// Given the current game's state it will return the next one, for example,
    /// if `GameState::MainMenu` was passed in and the Start button was pressed `GameState::ChooseDifficulty` will be returned.
    ///
    /// Should be called on the update window event.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate poke_a_mango;
    /// # extern crate conrod;
    /// # use poke_a_mango::ops::{GameState, Widgets};
    /// # struct Event;
    /// # impl Event {
    /// #     pub fn update<F: FnOnce(())>(&self, f: F) {
    /// #         f(());
    /// #     }
    /// # }
    /// # fn main() {
    /// # let mut ui = conrod::UiBuilder::new().build();
    /// # let widgets = Widgets::new(ui.widget_id_generator());
    /// # let event = Event;
    /// let mut game_state = GameState::MainMenu;
    /// event.update(|_| {
    ///     game_state = widgets.update(ui.set_widgets(), game_state)
    /// });
    /// # }
    /// ```
    pub fn update(&self, mut ui_wdgts: UiCell, cur_state: GameState) -> GameState {
        match cur_state {
            GameState::MainMenu => {
                Canvas::new()
                    .flow_down(&[(self.start_button_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.leaderboard_button_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.exit_button_canvas, Canvas::new().color(DARK_CHARCOAL))])
                    .set(self.main_canvas, &mut ui_wdgts);

                let mut start_button = Widgets::padded_butan("Start", self.start_button_canvas);
                let mut leaderboard_button = Widgets::padded_butan("Leaderboard", self.leaderboard_button_canvas);
                let mut exit_button = Widgets::padded_butan("Exit", self.exit_button_canvas);
                set_button_style(&mut start_button);
                set_button_style(&mut leaderboard_button);
                set_button_style(&mut exit_button);

                if start_button.set(self.start_button, &mut ui_wdgts).was_clicked() {
                    GameState::ChooseDifficulty
                } else if leaderboard_button.set(self.leaderboard_button, &mut ui_wdgts).was_clicked() {
                    GameState::DisplayLeaderboard
                } else if exit_button.set(self.exit_button, &mut ui_wdgts).was_clicked() {
                    GameState::Exit
                } else {
                    GameState::MainMenu
                }
            }
            GameState::ChooseDifficulty => {
                Canvas::new()
                    .flow_down(&[(self.easy_button_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.normal_button_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.hard_button_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.back_button_canvas, Canvas::new().color(DARK_CHARCOAL))])
                    .set(self.main_canvas, &mut ui_wdgts);

                let mut easy_button = Widgets::padded_butan("Easy", self.easy_button_canvas);
                let mut normal_button = Widgets::padded_butan("Normal", self.normal_button_canvas);
                let mut hard_button = Widgets::padded_butan("Hard", self.hard_button_canvas);
                let mut back_button = Widgets::padded_butan("Back", self.back_button_canvas);
                set_button_style(&mut easy_button);
                set_button_style(&mut normal_button);
                set_button_style(&mut hard_button);
                set_button_style(&mut back_button);

                if easy_button.set(self.easy_button, &mut ui_wdgts).was_clicked() {
                    GameState::Playing(Difficulty::Easy)
                } else if normal_button.set(self.normal_button, &mut ui_wdgts).was_clicked() {
                    GameState::Playing(Difficulty::Normal)
                } else if hard_button.set(self.hard_button, &mut ui_wdgts).was_clicked() {
                    GameState::Playing(Difficulty::Hard)
                } else if back_button.set(self.back_button, &mut ui_wdgts).was_clicked() {
                    GameState::MainMenu
                } else {
                    GameState::ChooseDifficulty
                }
            }
            s => s,
        }
    }

    fn padded_butan(label: &'static str, canvas: Id) -> Button<'static, Flat> {
        Button::new()
            .label(label)
            .padded_wh_of(canvas, 20.0)
            .mid_top_with_margin_on(canvas, 20.0)
    }
}

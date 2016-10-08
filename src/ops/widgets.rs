use conrod::{Positionable, Borderable, Colorable, Labelable, Sizeable, Widget, UiCell, Align};
use conrod::widget::text_box::{TextBox, Event as TextBoxEvent};
use self::super::{Difficulty, GameState, set_button_style};
use conrod::widget::button::{Button, Flat};
use conrod::color::{DARK_CHARCOAL, WHITE};
use self::super::super::util::fruit_name;
use conrod::widget::id::{Generator, Id};
use conrod::widget::{Canvas, Text};
use self::super::state;
use std::cmp;


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
///     widgets.update(ui.set_widgets(), &mut game_state);
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

    top_label_canvas: Id,
    username_score_canvases: [Id; 10],

    top_label: Id,
    usernames: [Id; 10],
    scores: [Id; 10],

    mango_button_canvas: Id,
    mango_button: Id,

    score_label_canvas: Id,
    name_input_canvas: Id,
    done_button_canvas: Id,

    score_label: Id,
    name_input: Id,
    done_button: Id,
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
            top_label_canvas: id_gen.next(),
            username_score_canvases: [id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next(),
                                      id_gen.next()],
            top_label: id_gen.next(),
            usernames: [id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next(),
                        id_gen.next()],
            scores: [id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next(),
                     id_gen.next()],
            mango_button_canvas: id_gen.next(),
            mango_button: id_gen.next(),
            score_label_canvas: id_gen.next(),
            name_input_canvas: id_gen.next(),
            done_button_canvas: id_gen.next(),
            score_label: id_gen.next(),
            name_input: id_gen.next(),
            done_button: id_gen.next(),
        }
    }

    /// Update the UI elements and set them.
    ///
    /// Given the current game's state it will update it with the next one, if needed, for example,
    /// if `GameState::MainMenu` was passed in and the Start button was pressed it'll be updated to
    /// `GameState::ChooseDifficulty`.
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
    ///     widgets.update(ui.set_widgets(), &mut game_state)
    /// });
    /// # }
    /// ```
    pub fn update(&self, mut ui_wdgts: UiCell, mut cur_state: &mut GameState) {
        match *cur_state {
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
                    state::press_start(cur_state);
                } else if leaderboard_button.set(self.leaderboard_button, &mut ui_wdgts).was_clicked() {
                    state::press_display_highscores(cur_state);
                } else if exit_button.set(self.exit_button, &mut ui_wdgts).was_clicked() {
                    state::press_exit(cur_state);
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
                    state::select_difficulty(cur_state, Difficulty::Easy);
                } else if normal_button.set(self.normal_button, &mut ui_wdgts).was_clicked() {
                    state::select_difficulty(cur_state, Difficulty::Normal);
                } else if hard_button.set(self.hard_button, &mut ui_wdgts).was_clicked() {
                    state::select_difficulty(cur_state, Difficulty::Hard);
                } else if back_button.set(self.back_button, &mut ui_wdgts).was_clicked() {
                    state::press_back(cur_state);
                }
            }
            GameState::Playing { .. } => {
                Canvas::new()
                    .flow_down(&[(self.top_label_canvas, Canvas::new().color(DARK_CHARCOAL)), (self.mango_button_canvas, Canvas::new().color(DARK_CHARCOAL))])
                    .set(self.main_canvas, &mut ui_wdgts);

                let last_fruit = state::tick_mango(cur_state);

                Widgets::paddded_text("Poke a mango!", self.top_label_canvas, Align::Middle).set(self.top_label, &mut ui_wdgts);
                let mut mango_button = Widgets::padded_butan(fruit_name(&last_fruit), self.mango_button_canvas);
                set_button_style(&mut mango_button);

                let mango_button_clicked = mango_button.set(self.mango_button, &mut ui_wdgts).was_clicked();
                state::end_mango(cur_state, mango_button_clicked, last_fruit.is_none());
            }
            GameState::GameOver { score, .. } => {
                Canvas::new()
                    .flow_down(&[(self.top_label_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.score_label_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.name_input_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.done_button_canvas, Canvas::new().color(DARK_CHARCOAL)),
                                 (self.back_button_canvas, Canvas::new().color(DARK_CHARCOAL))])
                    .set(self.main_canvas, &mut ui_wdgts);

                Widgets::paddded_text("Game over", self.top_label_canvas, Align::Middle).set(self.top_label, &mut ui_wdgts);
                Widgets::paddded_text(&format!("Score: {}", score), self.score_label_canvas, Align::Middle).set(self.score_label, &mut ui_wdgts);

                let (mut pressed, correct_name) = if let GameState::GameOver { ref mut name, .. } = *cur_state {
                    let name_c = name.clone();
                    let name_input = TextBox::new(&name_c)
                        .color(DARK_CHARCOAL)
                        .text_color(WHITE)
                        .border(0.0)
                        .padded_w_of(self.name_input_canvas, 20.0)
                        .mid_left_with_margin_on(self.name_input_canvas, 20.0);

                    let mut pressed = false;
                    for ev in name_input.set(self.name_input, &mut ui_wdgts) {
                        match ev {
                            TextBoxEvent::Update(iname) => *name = iname,
                            TextBoxEvent::Enter => pressed = true,
                        }
                    }
                    (pressed, name != "" && name != "Your name")
                } else {
                    (false, false)
                };

                let mut done_button = Widgets::padded_butan("Done", self.done_button_canvas);
                set_button_style(&mut done_button);
                pressed = pressed || done_button.set(self.done_button, &mut ui_wdgts).was_clicked();

                let mut back_button = Widgets::padded_butan("Back", self.back_button_canvas);
                set_button_style(&mut back_button);
                let back_pressed = back_button.set(self.back_button, &mut ui_wdgts).was_clicked();

                if pressed && correct_name {
                    state::submit_name(cur_state);
                } else if back_pressed {
                    state::press_back(cur_state);
                }
            }
            GameState::DisplayLeaderboard(_) => {
                if let GameState::DisplayLeaderboard(ref ldrbrd) = *cur_state {
                    let leader_n = cmp::min(ldrbrd.len(), 10);

                    let mut canvases = Vec::with_capacity(leader_n + 2);
                    canvases.push((self.top_label_canvas, Canvas::new().color(DARK_CHARCOAL)));
                    canvases.append(&mut (0..leader_n).map(|i| (self.username_score_canvases[i], Canvas::new().color(DARK_CHARCOAL))).collect());
                    canvases.push((self.back_button_canvas, Canvas::new().color(DARK_CHARCOAL)));
                    Canvas::new()
                        .flow_down(&canvases)
                        .set(self.main_canvas, &mut ui_wdgts);

                    Widgets::paddded_text("Leaderboard", self.top_label_canvas, Align::Middle).set(self.top_label, &mut ui_wdgts);

                    for i in 0..leader_n {
                        Widgets::paddded_text(&ldrbrd[i].name, self.username_score_canvases[i], Align::Start).set(self.usernames[i], &mut ui_wdgts);
                        Widgets::paddded_text(&ldrbrd[i].score.to_string(), self.username_score_canvases[i], Align::End).set(self.scores[i], &mut ui_wdgts);
                    }
                }

                let mut back_button = Widgets::padded_butan("Back", self.back_button_canvas);
                set_button_style(&mut back_button);
                if back_button.set(self.back_button, &mut ui_wdgts).was_clicked() {
                    state::press_back(cur_state);
                }
            }
            GameState::GameEnded { .. } |
            GameState::LoadLeaderboard |
            GameState::Exit => {
                Canvas::new().color(DARK_CHARCOAL).set(self.main_canvas, &mut ui_wdgts);
            }
        }
    }

    fn padded_butan(label: &'static str, canvas: Id) -> Button<'static, Flat> {
        Button::new()
            .label(label)
            .padded_wh_of(canvas, 20.0)
            .mid_top_with_margin_on(canvas, 20.0)
    }

    fn paddded_text(label: &str, canvas: Id, alignment: Align) -> Text {
        Text::new(label)
            .color(WHITE)
            .padded_w_of(canvas, 5.0)
            .mid_left_with_margin_on(canvas, 5.0)
            .line_spacing(2.0)
            .align_text_to(alignment)
    }
}

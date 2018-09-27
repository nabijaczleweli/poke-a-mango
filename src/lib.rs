//! What all the kool kidz are playing these days
//!
//! ## Features
//!
//! Easily navigable menus!
//!
//! ![Start menu screenshot](https://rawcdn.githack.com/nabijaczleweli/poke-a-mango/master/assets/screenshot_start_menu.png)
//!
//! Extensive leaderboard system!
//!
//! ![Leaderboard screenshot](https://rawcdn.githack.com/nabijaczleweli/poke-a-mango/master/assets/screenshot_leaderboard.png)
//!
//! A wide variety of difficulty modes!
//!
//! ![Difficulty selection
//! screenshot](https://rawcdn.githack.com/nabijaczleweli/poke-a-mango/master/assets/screenshot_difficulty_selection.png)
//!
//! Exciting gameplay that will have you glued to the screen for hours on end!
//!
//! ![Gameplay screenshot](https://rawcdn.githack.com/nabijaczleweli/poke-a-mango/master/assets/screenshot_gameplay.png)
//!
//! # Library doc
//!
//! This library is used by `poke-a-mango` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! #### With `conrod`
//!
//! ```plaintext
//! Options::parse()
//! |> create_window()
//! |> Widgets::new()
//! ```
//!
//! Then, each update event:
//!
//! ```plaintext
//! |> Widgets::update()
//! |> [check for states requiring usercode handling and act accordingly]
//! ```
//!
//! #### With a different UI
//!
//! ```plaintext
//! Options::parse()
//! |> state::*()
//! |> [check for states requiring usercode handling and act accordingly]
//! ```
//!
//! ### Prose explanation
//!
//! First, get an `Options` instance, be it via a struct-literal or `Options::parse()`;
//! or don't and just create the individual arguments manually.
//!
//! Then, use `create_window()` and a new `conrod::Ui`, follow up with creating a new `Widgets` instance.
//!
//! After that, among normally displaying and processing events with `conrod::Ui` call `Widgets::update()` each update event.
//! That sometimes produces `GameState`s that need action from you and you need to handle them manually.
//!
//! ### Example
//!
//! This is a semi-complete example, as it lacks event handling, rendering and UI setup which have been omitted for brevity.
//!
//! ```no_run
//! # extern crate conrod;
//! # extern crate window;
//! # extern crate poke_a_mango;
//! # use window::Window;
//! # use conrod::backend::piston::event::UpdateEvent;
//! # use conrod::backend::piston::window::{WindowEvents, EventWindow, Window as PistonWindow};
//! # use poke_a_mango::*;
//! # fn main() {
//! #   not_main();
//! # }
//! # fn not_main() -> Result<(), Error> {
//! let opts = Options::parse();
//!
//! let mut window: PistonWindow = try!(ops::create_window(opts.desktop_size));
//! let window_s = ops::window_size(opts.desktop_size);
//! let mut ui = conrod::UiBuilder::new([window_s[0] as f64, window_s[1] as f64]).build();
//! // Set up the UI like normal
//!
//! let mut game_state = ops::GameState::MainMenu;
//! let widgets = ops::Widgets::new(ui.widget_id_generator());
//!
//! let mut events = WindowEvents::new();
//! while let Some(event) = window.next(&mut events) {
//!     event.update(|_| {
//!         widgets.update(ui.set_widgets(), &mut game_state);
//!
//!         if game_state.should_exit() {
//!             window.set_should_close(true);
//!         } else if game_state.should_load_leaderboard() {
//!             game_state =
//!                 ops::GameState::DisplayLeaderboard(ops::Leader::read(
//!                     &opts.config_dir.1.join("leaderboard.toml")).unwrap());
//!         } else if let ops::GameState::GameEnded { .. } = game_state {
//!             if let ops::GameState::GameEnded { ref name, score } = game_state {
//!                 ops::Leader::append(ops::Leader::now(name.clone(), score),
//!                                     &opts.config_dir.1.join("leaderboard.toml")).unwrap();
//!             }
//!             game_state = ops::GameState::MainMenu;
//!         }
//!     });
//!
//!     // Handle events and draw like you'd normally do
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Executable manpage
//!
//! Played with your friends or alone, this game provides great entertainment to
//! everyone all over the globe, requiring great eye-hand coordination and keeping
//! the atmosphere tense.
//!
//! Exit values and possible errors:
//!
//! ```text
//! 1 - Failed to parse a file
//! 2 - An I/O error occured
//! 3 - UI failed to cooperate
//! ```
//!
//! ## OPTIONS
//!
//! -c --config-dir &lt;config_dir&gt;
//!
//! ```text
//! Directory with the configuration.
//!
//! The configuration directory contains all of poke-a-mango's data.
//!
//! Default: $HOME/.poke-a-mango
//! ```
//!
//! -d --desktop-size &lt;size&gt;
//!
//! ```text
//! The target desktop's resolution.
//!
//! By default this is autodetected to match the primary monitor's resolution,
//! but can be overriden to scale the game window better.
//!
//! Format: NxM
//! ```
//!
//! ## EXAMPLES
//!
//! `poke-a-mango` `-d` *1280x720*
//!
//! ```text
//! Run the game as on an HD monitor.
//! ```
//!
//! `poke-a-mango` `-c` *pkmngo*
//!
//! ```text
//! Save the game data in "pkmngo" directory instead of the default one.
//! ```


extern crate rustc_serialize;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate conrod;
extern crate glutin;
extern crate regex;
#[macro_use]
extern crate clap;
extern crate toml;
extern crate rand;

mod error;
mod options;

pub mod ops;
pub mod util;

pub use error::Error;
pub use options::Options;

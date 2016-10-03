//! What all the kool kidz are playing these days
//!
//! # Library doc
//!
//! This library is used by `poke-a-mango` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
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
//! ### Prose explanation
//!
//! First, get an `Options` instance, be it via a struct-literal or `Options::parse()`;
//! or don't and just create the individual arguments manually.
//!
//! Then, use `ops::load_image()`. If you know your image's format, great. If you don't, get it via `ops::guess_format()`.
//!
//! After that resize the image to an output-ready size provided by `ops::image_resized_size()` with `resize_image()`.
//! `ops::image_resized_size()` takes into consideration using two pixels per cell in the output functions,
//! so the size it returns is twice as tall as the terminal output size passed to it.
//!
//! Finally, call `ops::write_ansi()`/`ops::write_ansi_truecolor()`/`ops::write_no_ansi()` depending on your liking with the
//! resulting image.
//!
//! Or, if you want to display images manually, use `ops::create_colourtable()` to create an approximate colours table and
//! display it, for example, with `ncurses`.
//!
//! ### Example
//!
//! This is a semi-complete example, the only thing it lacks is event handling and rendering.
//!
//! ```no_run
//! # extern crate conrod;
//! # extern crate poke_a_mango;
//! # extern crate piston_window;
//! # use piston_window::{PistonWindow, UpdateEvent, Window};
//! # use poke_a_mango::*;
//! # fn main() {
//! #   not_main();
//! # }
//! # fn not_main() -> Result<(), Error> {
//! let opts = Options::parse();
//!
//! let mut window: PistonWindow = try!(ops::create_window(opts.desktop_size));
//! let mut ui = conrod::UiBuilder::new().build();
//! // Set up the UI like normal
//!
//! let mut game_state = ops::GameState::MainMenu;
//! let widgets = ops::Widgets::new(ui.widget_id_generator());
//!
//! while let Some(event) = window.next() {
//!     event.update(|_| {
//!         widgets.update(ui.set_widgets(), &mut game_state);
//!
//!         if game_state.should_exit() {
//!             window.set_should_close(true);
//!         } else if game_state.should_load_leaderboard() {
//!             game_state =
//!                 ops::GameState::DisplayLeaderboard(ops::Leader::read(&opts.config_dir.1.join("leaderboard.toml")).unwrap());
//!         } else if let ops::GameState::GameEnded { .. } = game_state {
//!             if let ops::GameState::GameEnded { ref name, score } = game_state {
//!                 ops::Leader::append(ops::Leader::now(name.clone(), score), &opts.config_dir.1.join("leaderboard.toml")).unwrap();
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
//! -c --config-dir &lt;<config_dir>&gt;
//!
//! ```text
//! Directory with the configuration.
//!
//! The configuration directory contains all of poke-a-mango's data.
//!
//! Default: $HOME/.poke-a-mango
//! ```
//!
//! -d --desktop-size &lt;<size>&gt;
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
extern crate piston_window;
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

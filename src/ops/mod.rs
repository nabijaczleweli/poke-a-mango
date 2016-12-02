//! Main functions doing actual work.
//!
//! Use `create_window()` to set up a window of correct size, then set up a `conrod::Ui` on it like you'd do normally.
//! Then create an instance of `Widgets` and call `update()` on it each update event, then check for states that need manual
//! handling from usercode and act accordingly.


use toml::{Parser, Value, decode, encode_str};
use rustc_serialize::{Encodable, Decodable};
use std::io::{Read, Write};
use self::super::Error;
use std::path::Path;
use std::fs::File;

mod style;
mod leader;
mod window;
mod widgets;
mod game_state;

pub mod state;

pub use self::leader::Leader;
pub use self::widgets::Widgets;
pub use self::style::set_button_style;
pub use self::game_state::{Difficulty, GameState};
pub use self::window::{create_window, window_size};


fn read_toml_file<T: Decodable>(p: &Path, desc: &'static str) -> Result<T, Error> {
    let mut buf = String::new();
    try!(try!(File::open(p).map_err(|_| {
            Error::Io {
                desc: desc,
                op: "open",
            }
        }))
        .read_to_string(&mut buf)
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "read",
            }
        }));

    let mut parser = Parser::new(&buf);
    let parsed = parser.parse().and_then(|t| decode(Value::Table(t)));
    parsed.ok_or_else(move || {
        Error::FileParsingFailed {
            desc: desc,
            errors: parser.errors
                .iter()
                .map(|e| {
                    let (line, col) = parser.to_linecol(e.lo);
                    format!("error: {}:{}: {}", line, col, e.desc)
                })
                .collect(),
        }
    })
}

fn write_toml_file<T: Encodable>(to_write: &T, p: &Path, desc: &'static str) -> Result<(), Error> {
    try!(File::create(p).map_err(|_| {
            Error::Io {
                desc: desc,
                op: "create",
            }
        }))
        .write_all(encode_str(to_write).as_bytes())
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "write",
            }
        })
}

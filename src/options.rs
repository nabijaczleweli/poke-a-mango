//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use poke_a_mango::Options;
//! let options = Options::parse();
//! println!("Config directory: {}", options.config_dir.0);
//! ```


use clap::{self, App, Arg, AppSettings};
use std::path::PathBuf;
use std::env::home_dir;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory containing configuration. Default: `"$HOME/.poke-a-mango"`
    pub config_dir: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("poke-a-mango")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .about("What all the kool kidz are playing these days")
            .arg(Arg::from_usage("-c --config-dir=[CONFIG_DIR] 'Directory containing configuration. Default: $HOME/.poke-a-mango'")
                .validator(Options::config_dir_validator))
            .get_matches();

        Options {
            config_dir: match matches.value_of("config-dir") {
                Some(dirs) => (dirs.to_string(), fs::canonicalize(dirs).unwrap()),
                None => {
                    match home_dir() {
                        Some(mut hd) => {
                            hd = hd.canonicalize().unwrap();
                            hd.push(".poke-a-mango");

                            fs::create_dir_all(&hd).unwrap();
                            ("$HOME/.poke-a-mango".to_string(), hd)
                        }
                        None => {
                            clap::Error {
                                    message: "Couldn't automatically get home directory, please specify configuration directory with the -c option".to_string(),
                                    kind: clap::ErrorKind::MissingRequiredArgument,
                                    info: None,
                                }
                                .exit()
                        }
                    }
                }
            },
        }
    }

    fn config_dir_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Configuration directory \"{}\" not found", s))
    }
}

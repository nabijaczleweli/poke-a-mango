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
use glutin::get_primary_monitor;
use std::str::{self, FromStr};
use std::path::PathBuf;
use std::env::home_dir;
use regex::Regex;
use std::fs;


lazy_static! {
    static ref SIZE_ARG_RGX: Regex = Regex::new(r"(\d+)x(\d+)").unwrap();
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory containing configuration. Default: `"$HOME/.poke-a-mango"`
    pub config_dir: (String, PathBuf),
    /// The target monitor's resolution for scaling. Default: detected
    pub desktop_size: (u32, u32),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let detected_resolution_default = {
            let (w, h) = get_primary_monitor().get_dimensions();
            format!("{}x{}", w, h)
        };

        let matches = App::new("poke-a-mango")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .about("What all the kool kidz are playing these days")
            .arg(Arg::from_usage("-c --config-dir=[CONFIG_DIR] 'Directory containing configuration. Default: $HOME/.poke-a-mango'")
                .validator(Options::config_dir_validator))
            .arg(Arg::from_usage("-d --desktop-size=[DESKTOP_SIZE] 'The desktop's resolution'")
                .default_value(&detected_resolution_default)
                .validator(Options::size_validator))
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
            desktop_size: Options::parse_size(matches.value_of("desktop-size").unwrap()).unwrap(),
        }
    }

    fn parse_size(s: &str) -> Option<(u32, u32)> {
        SIZE_ARG_RGX.captures(s).map(|c| (u32::from_str(c.at(1).unwrap()).unwrap(), u32::from_str(c.at(2).unwrap()).unwrap()))
    }

    fn config_dir_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Configuration directory \"{}\" not found", s))
    }

    fn size_validator(s: String) -> Result<(), String> {
        match Options::parse_size(&s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _)) | Some((_, 0)) => Err(format!("Can't resize image to size 0")),
            Some(_) => Ok(()),
        }
    }
}

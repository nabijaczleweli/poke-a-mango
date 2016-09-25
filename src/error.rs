use self::super::util::uppercase_first;
use std::io::Write;


/// Enum representing all possible ways the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// Failed to parse the specified file because of the specified errors.
    FileParsingFailed {
        /// The file that failed to parse.
        desc: &'static str,
        /// The parsing errors that occured.
        errors: Vec<String>,
    },
    /// An I/O error occured.
    ///
    /// This includes higher-level I/O errors like FS ones.
    Io {
        /// The file that failed to parse.
        desc: &'static str,
        /// The failed operation.
        ///
        /// This should be lowercase and imperative ("create", "open").
        op: &'static str,
    },
    /// A UI error, like a failure to creat a Window.
    Ui {
        /// The UI operation's description.
        ///
        /// This should be imperative.
        desc: &'static str,
        /// The failed operation, as returned by the UI framework.
        error: String,
    },
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::iter::FromIterator;
    /// # use poke_a_mango::Error;
    /// let mut out = Vec::new();
    /// Error::Io {
    ///     desc: "leaderboard",
    ///     op: "write",
    /// }.print_error(&mut out);
    /// assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
    ///            "Writing leaderboard failed.\n".to_string());
    /// ```
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Error::FileParsingFailed { ref desc, ref errors } => {
                writeln!(err_out, "Failed to parse {}{}", desc, if errors.is_empty() { '.' } else { ':' }).unwrap();
                for err in errors {
                    writeln!(err_out, "  {}", err).unwrap()
                }
            }
            Error::Io { ref desc, ref op } => {
                // Strip the last 'e', if any, so we get correct inflection for continuous times
                let op = uppercase_first(if op.ends_with('e') {
                    &op[..op.len() - 1]
                } else {
                    op
                });
                writeln!(err_out, "{}ing {} failed.", op, desc).unwrap()
            }
            Error::Ui { ref desc, ref error } => writeln!(err_out, "Failed to {}: {}.", desc, error).unwrap(),
        }
    }

    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::process::exit;
    /// # use poke_a_mango::Error;
    /// assert_eq!(Error::FileParsingFailed {
    ///     desc: "",
    ///     errors: vec![],
    /// }.exit_value(), 1);
    /// ```
    pub fn exit_value(&self) -> i32 {
        match *self {
            Error::FileParsingFailed { .. } => 1,
            Error::Io { .. } => 2,
            Error::Ui { .. } => 3,
        }
    }
}

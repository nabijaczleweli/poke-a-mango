use std::io::Write;


/// Enum representing all possible values the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// Failed to parse the specified file because of the specified errors.
    FileParsingFailed {
        /// The file that failed to parse.
        desc: &'static str,
        /// The parsing errors that occured.
        errors: Vec<String>,
    },
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
    ///
    /// TODO: example on simple Error
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Error::FileParsingFailed { ref desc, ref errors } => {
                writeln!(err_out, "Failed to parse {}{}", desc, if errors.is_empty() { '.' } else { ':' }).unwrap();
                for err in errors {
                    writeln!(err_out, "  {}", err).unwrap()
                }
            }
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
        }
    }
}

//! Module containing various utility functions.


/// Default fruit names.
pub static FRUITS: &'static [&'static str] = &["Avocado", "Banana", "Melon", "Orange", "Pear", "Watermelon"];


/// Uppercase the first character of the supplied string.
///
/// Based on http://stackoverflow.com/a/38406885/2851815
///
/// # Examples
///
/// ```
/// # use poke_a_mango::util::uppercase_first;
/// assert_eq!(uppercase_first("abolish"), "Abolish".to_string());
/// ```
pub fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Get the fruit name from the format used in `GameStatus::Playing::fruit`.
///
/// # Examples
///
/// ```
/// # use poke_a_mango::util::fruit_name;
/// assert_eq!(fruit_name(&None), "Mango");
/// ```
pub fn fruit_name(fruit: &Option<usize>) -> &'static str {
    match *fruit {
        Some(i) => FRUITS[i],
        None => "Mango",
    }
}

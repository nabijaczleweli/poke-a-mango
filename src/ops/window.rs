use piston_window::{BuildFromWindowSettings, WindowSettings};
use self::super::super::Error;


/// Create a window based on the size of the target desktop.
///
/// Much like the raw equivalnt of creating the window, the return type is be dependent on the user-side's explicit
/// expression type.
///
/// Use `glutin::get_{primary,available}_monitor[s]()` to get the size of the respective desktops.
///
/// # Examples
///
/// Create a window for an HD monitor:
///
/// ```
/// # extern crate piston_window;
/// # extern crate poke_a_mango;
/// # use poke_a_mango::ops::create_window;
/// # use piston_window::NoWindow as PistonWindow;
/// # fn main() {
/// let window: PistonWindow = create_window((1280, 720)).unwrap();
/// # }
/// ```
pub fn create_window<W: BuildFromWindowSettings>(desktop_size: (u32, u32)) -> Result<W, Error> {
    let window_w = desktop_size.1 / 3;
    WindowSettings::new("poke-a-mango", [window_w, window_w * 2])
        .vsync(true)
        .build()
        .map_err(|e| {
            Error::Ui {
                desc: "create window",
                error: e,
            }
        })
}

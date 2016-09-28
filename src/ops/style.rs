use conrod::color::{TRANSPARENT, WHITE};
use conrod::widget::Button;


/// Set the button style that looks :+1: on the canvas.
///
/// # Examples
///
/// ```
/// # extern crate poke_a_mango;
/// # extern crate conrod;
/// # use poke_a_mango::ops::set_button_style;
/// # use conrod::widget::{Button, Id};
/// # use conrod::{UiBuilder, Widget};
/// # fn main() {
/// # let mut ui = UiBuilder::new().build();
/// # let mut ui_widgets = ui.set_widgets();
/// # let buttid = Id::new(1);
/// let mut butan = Button::new();
/// // Probably more positioning options
/// set_button_style(&mut butan);
///
/// butan.set(buttid, &mut ui_widgets);
/// # }
/// ```
pub fn set_button_style<'a, S>(button: &mut Button<'a, S>) {
    button.style.color = Some(TRANSPARENT);
    button.style.border = None;
    button.style.border_color = Some(TRANSPARENT);
    button.style.label_color = Some(WHITE);
}

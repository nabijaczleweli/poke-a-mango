use conrod::color::{DARK_CHARCOAL, TRANSPARENT, WHITE};
use conrod::{Colorable, Labelable, Widget, UiCell};
use conrod::widget::id::{Generator, Id};
use conrod::widget::{Canvas, Button};
use conrod::{Positionable, Sizeable};


/// Container for all widgets' IDs, also manages setting them.
///
/// The general idea is to call `Widgets::new()` once and then `Widgets::update()` each update event.
///
/// # Examples
///
/// ```
/// # extern crate poke_a_mango;
/// # extern crate conrod;
/// # use poke_a_mango::ops::Widgets;
/// # struct Event;
/// # impl Event {
/// #     pub fn update<F: FnOnce(())>(&self, f: F) {
/// #         f(());
/// #     }
/// # }
/// # fn main() {
/// let mut ui = conrod::UiBuilder::new().build();
/// let widgets = Widgets::new(ui.widget_id_generator());
///
/// // Then, in the event loop
/// # let event = Event;
/// event.update(|_| widgets.update(ui.set_widgets()));
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Widgets {
    main_canvas: Id,
    start_button: Id,
}

impl Widgets {
    /// Generate widget IDs in preparation for later setting them in `Ui`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate poke_a_mango;
    /// # extern crate conrod;
    /// # use poke_a_mango::ops::Widgets;
    /// # fn main() {
    /// let mut ui = conrod::UiBuilder::new().build();
    /// let widgets = Widgets::new(ui.widget_id_generator());
    /// # }
    /// ```
    pub fn new(mut id_gen: Generator) -> Widgets {
        Widgets {
            main_canvas: id_gen.next(),
            start_button: id_gen.next(),
        }
    }

    /// Update the UI elements and set them.
    ///
    /// Should be called on the update window event.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate poke_a_mango;
    /// # extern crate conrod;
    /// # use poke_a_mango::ops::Widgets;
    /// # struct Event;
    /// # impl Event {
    /// #     pub fn update<F: FnOnce(())>(&self, f: F) {
    /// #         f(());
    /// #     }
    /// # }
    /// # fn main() {
    /// # let mut ui = conrod::UiBuilder::new().build();
    /// # let widgets = Widgets::new(ui.widget_id_generator());
    /// # let event = Event;
    /// event.update(|_| widgets.update(ui.set_widgets()));
    /// # }
    /// ```
    pub fn update(&self, mut ui_wdgts: UiCell) {
        Canvas::new().color(DARK_CHARCOAL).set(self.main_canvas, &mut ui_wdgts);

        let mut butan = Button::new()
            .label("Start")
            .padded_wh_of(self.main_canvas, 20.0)
            .mid_top_with_margin_on(self.main_canvas, 20.0);
        butan.style.color = Some(TRANSPARENT);
        butan.style.border = None;
        butan.style.border_color = Some(TRANSPARENT);
        butan.style.label_color = Some(WHITE);
        butan.set(self.start_button, &mut ui_wdgts);
    }
}

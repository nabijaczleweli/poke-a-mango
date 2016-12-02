use conrod::widget::button::{Style as ButtonStyle, Button};
use poke_a_mango::ops::set_button_style;
use conrod::color::{TRANSPARENT, WHITE};


#[test]
fn button() {
    let mut butt = Button::new();
    set_button_style(&mut butt);

    assert_eq!(butt.style,
               ButtonStyle {
                   color: Some(TRANSPARENT),
                   border: None,
                   border_color: Some(TRANSPARENT),
                   label_color: Some(WHITE),
                   label_font_size: None,
                   label_x_align: None,
                   label_font_id: None,
               });
}

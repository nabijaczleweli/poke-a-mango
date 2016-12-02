use window::{AdvancedWindow, NoWindow, Window};
use poke_a_mango::ops::create_window;
use poke_a_mango::Error;


#[test]
fn resize_1600x900() {
    assert_eq!(window((1600, 900)).unwrap().size(), (300, 600).into());
}

#[test]
fn resize_1080p() {
    assert_eq!(window((1920, 1080)).unwrap().size(), (360, 720).into());
}

#[test]
fn resize_720p() {
    assert_eq!(window((1280, 720)).unwrap().size(), (240, 480).into());
}

#[test]
fn resize_pal() {
    assert_eq!(window((768, 576)).unwrap().size(), (192, 384).into());
}

#[test]
fn title() {
    assert_eq!(window((0, 0)).unwrap().get_title(), "poke-a-mango".to_string());
}

fn window(size: (u32, u32)) -> Result<NoWindow, Error> {
    create_window(size)
}

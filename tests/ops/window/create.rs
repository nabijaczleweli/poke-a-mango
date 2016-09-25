use piston_window::{AdvancedWindow, NoWindow, Window};
use poke_a_mango::ops::create_window;
use poke_a_mango::Error;


// TODO: replace explicit size destructuring assert_eq!s with a straight Size==Size once
//       https://github.com/PistonDevelopers/piston/pull/1120 gets released

#[test]
fn resize_1600x900() {
    let s = window((1600, 900)).unwrap().size();
    assert_eq!(s.width, 300);
    assert_eq!(s.height, 600);
}

#[test]
fn resize_1080p() {
    let s = window((1920, 1080)).unwrap().size();
    assert_eq!(s.width, 360);
    assert_eq!(s.height, 720);
}

#[test]
fn resize_720p() {
    let s = window((1280, 720)).unwrap().size();
    assert_eq!(s.width, 240);
    assert_eq!(s.height, 480);
}

#[test]
fn resize_pal() {
    let s = window((768, 576)).unwrap().size();
    assert_eq!(s.width, 192);
    assert_eq!(s.height, 384);
}

#[test]
fn title() {
    assert_eq!(window((0,0)).unwrap().get_title(), "poke-a-mango".to_string());
}

fn window(size: (u32, u32)) -> Result<NoWindow, Error> {
    create_window(size)
}

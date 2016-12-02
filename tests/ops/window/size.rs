use poke_a_mango::ops::window_size;


#[test]
fn size_1600x900() {
    assert_eq!(window_size((1600, 900)), [300, 600]);
}

#[test]
fn size_1080p() {
    assert_eq!(window_size((1920, 1080)), [360, 720]);
}

#[test]
fn size_720p() {
    assert_eq!(window_size((1280, 720)), [240, 480]);
}

#[test]
fn size_pal() {
    assert_eq!(window_size((768, 576)), [192, 384]);
}

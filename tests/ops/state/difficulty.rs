use poke_a_mango::ops::Difficulty;


#[test]
fn numeric() {
    assert_eq!(Difficulty::Easy.numeric(), 1);
    assert_eq!(Difficulty::Normal.numeric(), 2);
    assert_eq!(Difficulty::Hard.numeric(), 3);
}

#[test]
fn point_weight() {
    assert_eq!(Difficulty::Easy.point_weight(), 0.5);
    assert_eq!(Difficulty::Normal.point_weight(), 1.0);
    assert_eq!(Difficulty::Hard.point_weight(), 2.0);
}

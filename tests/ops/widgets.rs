use conrod::widget::id::Generator;
use poke_a_mango::ops::Widgets;
use conrod::graph::Graph;


#[test]
fn new_eq() {
    assert_eq!(Widgets::new(Generator::new(&mut Graph::new())), Widgets::new(Generator::new(&mut Graph::new())));
}

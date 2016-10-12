extern crate poke_a_mango;
extern crate cursive;

use std::io::stderr;
use cursive::Cursive;
use std::process::exit;
use cursive::views::{LinearLayout, Button};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), poke_a_mango::Error> {
    let opts = poke_a_mango::Options::parse();

    let mut ui = Cursive::new();
    ui.set_fps(60);

    let main_menu_screen = ui.add_screen();
    let choose_difficulty_screen = ui.add_screen();
    let highscores_screen = ui.add_screen();

    ui.set_screen(highscores_screen);
    // TODO: load the leaderboard
    ui.add_layer(LinearLayout::vertical().child(Button::new("Back", move |ui| ui.set_screen(main_menu_screen))));

    ui.set_screen(choose_difficulty_screen);
    // TODO: difficulty selection
    ui.add_layer(LinearLayout::vertical().child(Button::new("Back", move |ui| ui.set_screen(main_menu_screen))));

    ui.set_screen(main_menu_screen);
    ui.add_layer(LinearLayout::vertical()
        .child(Button::new("Start game", move |ui| ui.set_screen(choose_difficulty_screen)))
        .child(Button::new("Highscores", move |ui| ui.set_screen(highscores_screen)))
        .child(Button::new("Exit", Cursive::quit)));

    ui.run();

    Ok(())
}

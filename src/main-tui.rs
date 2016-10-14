extern crate poke_a_mango;
extern crate tabwriter;
extern crate cursive;

use std::cmp;
use std::path::Path;
use std::process::exit;
use tabwriter::TabWriter;
use std::io::{Write, stderr};
use cursive::{ScreenId, Cursive};
use cursive::views::{LinearLayout, TextView, Button};


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

    setup_ui((ui.add_screen(), ui.add_screen(), ui.add_screen()), &mut ui, &opts.config_dir.1);

    ui.run();

    Ok(())
}

fn setup_ui(screens: (ScreenId, ScreenId, ScreenId), ui: &mut Cursive, p: &Path) {
    let (main_menu_screen, choose_difficulty_screen, highscores_screen) = screens;

    // ui.set_screen(highscores_screen);
    // TODO: load the leaderboard
    // ui.add_layer(LinearLayout::vertical().child(Button::new("Back", move |ui| ui.set_screen(main_menu_screen))));

    ui.set_screen(choose_difficulty_screen);
    // TODO: difficulty selection
    ui.add_layer(LinearLayout::vertical().child(Button::new("Back", move |ui| ui.set_screen(main_menu_screen))));

    ui.set_screen(main_menu_screen);
    ui.add_layer(LinearLayout::vertical()
        .child(Button::new("Start game", move |ui| ui.set_screen(choose_difficulty_screen)))
        .child(Button::new("Highscores", {
            let p = p.to_path_buf();
            move |ui| setup_leaderboard(highscores_screen, main_menu_screen, ui, &p)
        }))
        .child(Button::new("Exit", Cursive::quit)));
}

fn setup_leaderboard(screen: ScreenId, back_screen: ScreenId, ui: &mut Cursive, p: &Path) {
    ui.set_screen(screen);
    let screen = ui.screen_mut();
    screen.pop_layer();

    let ldrbrd = leaderboard_to_string_form(poke_a_mango::ops::Leader::read(&p.join("leaderboard.toml")).expect("Failed to load leaderboard"));
    write!(&mut stderr(), "{:?}", ldrbrd).unwrap();
    let mut layout = LinearLayout::vertical();
    for ldr in ldrbrd.into_iter() {
        layout.add_child(TextView::new(ldr));
    }
    layout.add_child(Button::new("Back", move |ui| ui.set_screen(back_screen)));

    screen.add_layer(layout);
}

fn leaderboard_to_string_form(ldrbrd: Vec<poke_a_mango::ops::Leader>) -> Vec<String> {
    let mut tw = TabWriter::new(vec![]);

    let leader_n = cmp::min(ldrbrd.len(), 10);
    for ldr in ldrbrd.into_iter().take(leader_n) {
        writeln!(&mut tw, "{}\t{}", ldr.name, ldr.score).unwrap();
    }

    tw.flush().unwrap();
    String::from_utf8(tw.unwrap()).unwrap().lines().map(str::to_string).collect()
}

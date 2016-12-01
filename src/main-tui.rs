#[cfg(target_os="windows")]
#[link(name="poke-a-mango-manifest", kind="static")]
extern "C" {}


extern crate poke_a_mango;
extern crate tabwriter;
extern crate cursive;

use std::cmp;
use std::path::Path;
use std::str::FromStr;
use std::process::exit;
use tabwriter::TabWriter;
use cursive::view::Selector;
use std::io::{Write, stderr};
use cursive::{ScreenId, Cursive};
use cursive::traits::Identifiable;
use cursive::views::{LinearLayout, EditView, TextView, Button};


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

    setup_ui((ui.add_screen(), ui.add_screen(), ui.add_screen(), ui.add_screen(), ui.add_screen(), ui.add_screen()),
             &mut ui,
             &opts.config_dir.1);

    ui.run();

    Ok(())
}

fn setup_ui(screens: (ScreenId, ScreenId, ScreenId, ScreenId, ScreenId, ScreenId), ui: &mut Cursive, p: &Path) {
    let (main_menu_screen, choose_difficulty_screen, highscores_screen, play_game_screen, game_over_screen, data_save_screen) = screens;

    ui.set_screen(choose_difficulty_screen);
    ui.add_layer(LinearLayout::vertical()
        .child(Button::new("Easy", move |ui| {
            ui.set_screen(data_save_screen);
            difficulty_to_ui(ui, data_save_screen, play_game_screen, poke_a_mango::ops::Difficulty::Easy);
        }))
        .child(Button::new("Normal", move |ui| {
            ui.set_screen(data_save_screen);
            difficulty_to_ui(ui, data_save_screen, play_game_screen, poke_a_mango::ops::Difficulty::Normal);
        }))
        .child(Button::new("Hard", move |ui| {
            ui.set_screen(data_save_screen);
            difficulty_to_ui(ui, data_save_screen, play_game_screen, poke_a_mango::ops::Difficulty::Hard);
        }))
        .child(Button::new("Back", move |ui| ui.set_screen(main_menu_screen))));

    ui.set_screen(data_save_screen);
    ui.add_layer(TextView::new("").with_id("fruit"));
    ui.add_layer(TextView::new("0").with_id("score"));
    ui.add_layer(TextView::new("0").with_id("difficulty"));

    ui.set_screen(play_game_screen);
    ui.add_layer(LinearLayout::vertical()
        .child(TextView::new("Poke a mango!").center())
        .child(TextView::new("Current fruit: Mango").with_id("mango_label"))
        .child(Button::new("Poke it!", {
            let p = p.to_path_buf();
            move |ui| {
                let was_mango = mango_butan_press_common(ui, data_save_screen, play_game_screen);

                if was_mango {
                    update_score_in_ui(ui, data_save_screen, play_game_screen, 1);
                } else {
                    setup_game_over(game_over_screen, data_save_screen, main_menu_screen, ui, &p);
                }
            }
        }))
        .child(Button::new("Ignore it", move |ui| {
            mango_butan_press_common(ui, data_save_screen, play_game_screen);
        })));

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
    let mut layout = LinearLayout::vertical();
    for ldr in ldrbrd.into_iter() {
        layout.add_child(TextView::new(ldr));
    }
    layout.add_child(Button::new("Back", move |ui| ui.set_screen(back_screen)));

    screen.add_layer(layout);
}

fn setup_game_over(screen: ScreenId, data_screen: ScreenId, back_screen: ScreenId, ui: &mut Cursive, p: &Path) {
    let score = update_score_in_ui(ui, data_screen, screen, 0);

    let screen = ui.screen_mut();
    screen.pop_layer();

    let pe = p.to_path_buf();
    let pd = p.to_path_buf();
    screen.add_layer(LinearLayout::vertical()
        .child(TextView::new("Game over!"))
        .child(TextView::new(format!("Score: {}", score)))
        .child(EditView::new()
            .content("Your name")
            .on_submit(move |ui, name| finish_game(ui, data_screen, back_screen, &pe, name.to_string(), score))
            .with_id("name_edit"))
        .child(Button::new("Done", move |ui| {
            let name = {
                let ref mut name: &mut EditView = ui.find(&Selector::Id("name_edit")).unwrap();
                name.get_content().as_ref().clone()
            };
            finish_game(ui, data_screen, back_screen, &pd, name, score);
        }))
        .child(Button::new("Back", move |ui| reset_game_data(ui, data_screen, back_screen))));
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

fn fruit_from_ui(ui: &mut Cursive, data: ScreenId, back: ScreenId) -> Option<usize> {
    ui.set_screen(data);
    let fruit = {
        let ref mut fruit: &mut TextView = ui.find(&Selector::Id("fruit")).unwrap();
        match fruit.get_content() {
            "" => None,
            l => Some(usize::from_str(l).unwrap()),
        }
    };
    ui.set_screen(back);
    fruit
}

fn fruit_to_ui(ui: &mut Cursive, data: ScreenId, back: ScreenId, new_fruit: Option<usize>) {
    ui.set_screen(data);
    {
        let ref mut fruit: &mut TextView = ui.find(&Selector::Id("fruit")).unwrap();
        fruit.set_content(new_fruit.as_ref().map(usize::to_string).unwrap_or(String::new()));
    }
    ui.set_screen(back);
}

fn difficulty_from_ui(ui: &mut Cursive, data: ScreenId, back: ScreenId) -> poke_a_mango::ops::Difficulty {
    ui.set_screen(data);
    let difficulty = {
        let ref mut difficulty: &mut TextView = ui.find(&Selector::Id("difficulty")).unwrap();
        poke_a_mango::ops::Difficulty::from_numeric(u32::from_str(difficulty.get_content()).unwrap()).unwrap()
    };
    ui.set_screen(back);
    difficulty
}

fn difficulty_to_ui(ui: &mut Cursive, data: ScreenId, back: ScreenId, dfc: poke_a_mango::ops::Difficulty) {
    ui.set_screen(data);
    {
        let ref mut difficulty: &mut TextView = ui.find(&Selector::Id("difficulty")).unwrap();
        difficulty.set_content(dfc.numeric().to_string());
    }
    ui.set_screen(back);
}

fn update_score_in_ui(ui: &mut Cursive, data: ScreenId, back: ScreenId, by: u64) -> u64 {
    ui.set_screen(data);
    let score = {
        let ref mut score: &mut TextView = ui.find(&Selector::Id("score")).unwrap();
        let score_n = u64::from_str(score.get_content()).unwrap() + by;
        score.set_content(score_n.to_string());
        score_n
    };
    ui.set_screen(back);
    score
}

fn mango_butan_press_common(ui: &mut Cursive, data: ScreenId, back: ScreenId) -> bool {
    let fruit = fruit_from_ui(ui, data, back);
    let difficulty = difficulty_from_ui(ui, data, back);

    let mut state = poke_a_mango::ops::GameState::Playing {
        difficulty: difficulty,
        score: 0,
        fruit: fruit,
    };
    poke_a_mango::ops::state::tick_mango(&mut state);
    if let poke_a_mango::ops::GameState::Playing { fruit, .. } = state {
        fruit_to_ui(ui, data, back, fruit);

        let ref mut mango_label: &mut TextView = ui.find(&Selector::Id("mango_label")).expect("mango_label");
        mango_label.set_content(format!("Current fruit: {}", poke_a_mango::util::fruit_name(&fruit)));
    }

    fruit.is_none()
}

fn finish_game(ui: &mut Cursive, data: ScreenId, back: ScreenId, p: &Path, name: String, score: u64) {
    let mut s = poke_a_mango::ops::GameState::GameOver {
        difficulty: difficulty_from_ui(ui, data, data),
        score: score,
        name: name,
    };
    poke_a_mango::ops::state::submit_name(&mut s);
    if let poke_a_mango::ops::GameState::GameEnded { name, score } = s {
        poke_a_mango::ops::Leader::append(poke_a_mango::ops::Leader::now(name, score), &p.join("leaderboard.toml"))
            .expect("Failed to add leader to leaderboard");
    }

    reset_game_data(ui, data, back);
}

fn reset_game_data(ui: &mut Cursive, data: ScreenId, back: ScreenId) {
    ui.set_screen(data);
    {
        let ref mut fruit: &mut TextView = ui.find(&Selector::Id("fruit")).unwrap();
        fruit.set_content("");
    }
    {
        let ref mut score: &mut TextView = ui.find(&Selector::Id("score")).unwrap();
        score.set_content("0");
    }
    ui.set_screen(back);
}

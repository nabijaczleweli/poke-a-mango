extern crate piston_window;
extern crate poke_a_mango;
extern crate rusttype;
extern crate conrod;

use std::io::stderr;
use std::process::exit;
use rusttype::FontCollection;
use conrod::backend::piston_window as conrod_backend;
use piston_window::{PistonWindow, UpdateEvent, Window};


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

    let mut window: PistonWindow = try!(poke_a_mango::ops::create_window(opts.desktop_size));
    let mut ui = conrod::UiBuilder::new().build();
    let mut glyph_cache = conrod_backend::GlyphCache::new(&mut window, opts.desktop_size.0, opts.desktop_size.1);
    let image_map = conrod::image::Map::new();

    ui.fonts.insert(FontCollection::from_bytes(&include_bytes!("../assets/DejaVuSansMono.ttf")[..]).into_font().unwrap());

    let mut game_state = poke_a_mango::ops::GameState::MainMenu;
    let widgets = poke_a_mango::ops::Widgets::new(ui.widget_id_generator());

    while let Some(event) = window.next() {
        if let Some(e) = conrod_backend::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        let is_update = event.update(|_| {
                widgets.update(ui.set_widgets(), &mut game_state);
            })
            .is_some();
        if is_update {
            if game_state.should_exit() {
                window.set_should_close(true);
            } else if game_state.should_load_leaderboard() {
                game_state =
                    poke_a_mango::ops::GameState::DisplayLeaderboard(try!(poke_a_mango::ops::Leader::read(&opts.config_dir.1.join("leaderboard.toml"))));
            } else if let poke_a_mango::ops::GameState::GameEnded { name, score } = game_state {
                let leaderboard_p = opts.config_dir.1.join("leaderboard.toml");
                let mut leaderboard = try!(poke_a_mango::ops::Leader::read(&leaderboard_p));
                leaderboard.push(poke_a_mango::ops::Leader::now(name, score));
                leaderboard.sort();
                leaderboard.reverse();
                try!(poke_a_mango::ops::Leader::write(leaderboard, &leaderboard_p));

                game_state = poke_a_mango::ops::GameState::MainMenu;
            }
        }

        window.draw_2d(&event, |ctx, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {
                conrod_backend::draw(ctx, graphics, primitives, &mut glyph_cache, &image_map, |img| img);
            }
        });
    }

    Ok(())
}

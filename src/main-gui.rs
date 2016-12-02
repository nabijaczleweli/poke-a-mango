#[cfg(target_os="windows")]
#[link(name="poke-a-mango-manifest", kind="static")]
extern "C" {}


extern crate poke_a_mango;
extern crate rusttype;
extern crate conrod;
extern crate window;

use window::Window;
use std::io::stderr;
use std::process::exit;
use rusttype::FontCollection;
use conrod::backend::piston::event::UpdateEvent;
use conrod::backend::piston::window::{self as conrod_backend, WindowEvents, EventWindow, Window as PistonWindow};


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
    let mut ui = conrod::UiBuilder::new({
            let win_s = poke_a_mango::ops::window_size(opts.desktop_size);
            [win_s[0] as f64, win_s[1] as f64]
        })
        .build();
    let mut glyph_cache = conrod_backend::GlyphCache::new(&mut window, opts.desktop_size.0, opts.desktop_size.1);
    let image_map = conrod::image::Map::new();

    ui.fonts.insert(FontCollection::from_bytes(&include_bytes!("../assets/DejaVuSansMono.ttf")[..]).into_font().unwrap());

    let mut game_state = poke_a_mango::ops::GameState::MainMenu;
    let widgets = poke_a_mango::ops::Widgets::new(ui.widget_id_generator());

    let mut events = WindowEvents::new();
    while let Some(event) = window.next(&mut events) {
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
                try!(poke_a_mango::ops::Leader::append(poke_a_mango::ops::Leader::now(name, score), &opts.config_dir.1.join("leaderboard.toml")));
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

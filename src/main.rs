extern crate piston_window;
extern crate poke_a_mango;
extern crate conrod;

use std::io::stderr;
use std::process::exit;
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
    println!("{:#?}", opts);

    let mut window: PistonWindow = try!(poke_a_mango::ops::create_window(opts.desktop_size));
    let mut ui = conrod::UiBuilder::new().build();
    let mut glyph_cache = conrod_backend::GlyphCache::new(&mut window, opts.desktop_size.0, opts.desktop_size.1);
    let image_map = conrod::image::Map::new();

    ui.fonts.insert_from_file(opts.config_dir.1.join("assets").join("DejaVuSansMono.ttf")).unwrap();

    let mut game_state = poke_a_mango::ops::GameState::MainMenu;
    let widgets = poke_a_mango::ops::Widgets::new(ui.widget_id_generator());

    while let Some(event) = window.next() {
        if let Some(e) = conrod_backend::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            game_state = widgets.update(ui.set_widgets(), game_state);

            if game_state.should_exit() {
                window.set_should_close(true);
            }
        });

        window.draw_2d(&event, |ctx, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {
                conrod_backend::draw(ctx, graphics, primitives, &mut glyph_cache, &image_map, |img| img);
            }
        });
    }

    Ok(())
}

extern crate piston_window;
extern crate poke_a_mango;

use std::io::stderr;
use std::process::exit;
use piston_window::PistonWindow;


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
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            piston_window::clear([0.5, 0.5, 0.5, 1.0], g);
            piston_window::rectangle([1.0, 0.0, 0.0, 1.0], // red
                                     [0.0, 0.0, 100.0, 100.0], // rectangle
                                     c.transform,
                                     g);
        });
    }

    Ok(())
}

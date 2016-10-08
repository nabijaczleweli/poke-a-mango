extern crate poke_a_mango;
extern crate cursive;

use std::io::stderr;
use std::process::exit;
use cursive::Cursive;


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
    println!("{:?}", opts);

    let mut ui = Cursive::new();
    ui.set_fps(60);

    ui.add_global_callback('q', Cursive::quit);

    ui.run();

    Ok(())
}

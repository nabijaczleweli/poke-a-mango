use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    if cfg!(not(target_os = "windows")) {
        return;
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    if cfg!(target_env = "msvc") {
        // `.res`es are linkable under MSVC as well as normal libraries.
        Command::new("rc")
            .args(&["/fo", &format!("{}/poke-a-mango-manifest.lib", out_dir), "poke-a-mango-manifest.rc"])
            .status()
            .expect("Are you sure you have RC.EXE in your $PATH?");
    } else {
        Command::new("windres")
            .args(&["--input", "poke-a-mango-manifest.rc", "--output-format=coff", "--output"])
            .arg(&format!("{}/poke-a-mango-manifest.res", out_dir))
            .status()
            .unwrap();

        Command::new("ar")
            .args(&["crs", "libpoke-a-mango-manifest.a", "poke-a-mango-manifest.res"])
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
}

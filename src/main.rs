use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use app::app::App;
use app::config::WindowConfig;

mod app;
mod emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide a file to be loaded");
        std::process::exit(1);
    }

    let filename = &args[1];
    println!("The filename to load is: {}", filename);

    let mut app = match App::new(WindowConfig::default()) {
        Ok(app) => app,
        Err(err) => {
            eprintln!("Error creating App: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = app.load_rom(filename) {
        eprintln!("Error loading ROM: {}", err);
        std::process::exit(1);
    }

    while app.is_running() {
        app.process_input().unwrap();
        app.render();
    }
}

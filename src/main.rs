use app::App;

mod app;
mod emulator;

fn main() {
    let mut app = App::new("CHIP-8 emulator", 800, 600).unwrap();
    while app.is_running() {
        app.process_input().unwrap();
        app.render();
    }

}

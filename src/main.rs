use app::app::App;
use app::config::WindowConfig;

mod app;
mod emulator;

fn main() {
    let mut app = App::new(WindowConfig::default()).unwrap();
    while app.is_running() {
        app.process_input().unwrap();
        app.render();
    }
}

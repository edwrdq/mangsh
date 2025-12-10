mod app;
mod ui;
mod ssh;
mod config;

fn main() {
    let mut app = app::App::new();
    app.run();
}

mod app;
mod config;
mod theme;
mod ui;

fn main() -> anyhow::Result<()> {
    let mut app = app::App::new();
    app.run()
}

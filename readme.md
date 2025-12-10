🥭 README.md for mangsh
# 🥭 mangsh  
A terminal-based SSH client built in Rust, powered by Ratatui.

mangsh is an experimental TUI (text user interface) SSH manager designed to be simple, fast, and eventually integrated into the future mangOS ecosystem.  
It features a clean UI layout, a title screen, multiple app modes, and a foundation for configuration and host management.

> ⚠️ Work in progress. Most features are still prototypes or placeholders.

---

## 🚀 Features (Current)

- Custom TUI built with **ratatui**
- Clean UI architecture with:
  - `title` screen  
  - `main` screen  
  - `config` screen  
- Modular UI system (`layout.rs`, `panels.rs`, `ui/mod.rs`)
- Screen state machine using a Rust `enum`
- Basic keybinds:
  - `Enter` → continue past title screen
  - `c` → open config
  - `q` → quit

---

## 🧩 Project Structure



src/
main.rs -- entrypoint
app.rs -- App struct, screen enum, event loop
ui/
mod.rs -- root of UI drawing
layout.rs -- handles screen splits
panels.rs -- individual screens & panels


---

## 🛠 Building

Make sure you have Rust installed.  
Then:

```sh
cargo build
```

Run with:

cargo run

📦 Dependencies

ratatui → terminal UI library

crossterm → keyboard + terminal control

anyhow → easy error handling

serde + toml (optional) for config files

Your Cargo.toml should include:

```toml
[dependencies]
crossterm = "0.27"
anyhow = "1"
ratatui = { version = "0.29", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

🧠 Planned Features

Real SSH backend (RustCrypto + async executor)

Host profiles loaded from TOML

Theming + color schemes

Tor mode (SSH over onion routing)

Left panel host selector

Right panel live SSH view

Scrollback, logs, and status bars

Integration with mangOS + orchard package system

🤝 Contributing

mangsh is early-stage and experimental.
PRs are welcome once the base structure stabilizes.

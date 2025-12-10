use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::path::PathBuf;

use crate::{
    config::{self, Config},
    theme::{all_themes, Theme},
    ui,
};

//screen state enum -- title screen, main screen, config, etc... 
pub enum Screen {
    Title,
    Main,
    Config,
    Menu,
}

pub struct App{
    pub screen: Screen,
    pub tabs: Vec<&'static str>,
    pub tab_index: usize,
    pub theme: Theme,
    pub config: Config,
    pub config_path: Option<PathBuf>,
    pub config_error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let loaded = config::loader::load_config();
        let theme = loaded.config.effective_theme();

        Self {
            screen: Screen::Title,
            tabs: vec!["Hosts", "Session"],
            tab_index: 0,
            theme,
            config: loaded.config,
            config_path: loaded.path,
            config_error: loaded.error,
        }
    }

    fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tabs.len();
    }

    fn previous_tab(&mut self) {
        if self.tab_index == 0 {
            self.tab_index = self.tabs.len() - 1;
        } else {
            self.tab_index -= 1;
        }
    }

    pub fn next_theme(&mut self) {
        let themes = all_themes();
        if let Some(pos) = themes.iter().position(|t| *t == self.theme) {
            let next = (pos + 1) % themes.len();
            self.set_theme(themes[next]);
        }
    }

    pub fn previous_theme(&mut self) {
        let themes = all_themes();
        if let Some(pos) = themes.iter().position(|t| *t == self.theme) {
            let prev = if pos == 0 { themes.len() - 1 } else { pos - 1 };
            self.set_theme(themes[prev]);
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.config.theme = Some(theme);
        match config::loader::save_config(&self.config) {
            Ok(Some(path)) => {
                self.config_path = Some(path);
                self.config_error = None;
            }
            Ok(None) => {
                self.config_error = Some("No config directory; theme not persisted.".to_string());
            }
            Err(err) => {
                self.config_error = Some(format!("Failed to save config: {err}"));
            }
        };
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut terminal = ratatui::init();

        loop {
            // DRAW SCREEN
            terminal.draw(|frame| {
                ui::draw(frame, &self);
            })?;

            // CHECK FOR INPUT
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    // global exit
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                    match self.screen {
                        Screen::Title => {
                            match key.code {
                                KeyCode::Enter => {
                                    self.screen = Screen::Main;
                                    self.tab_index = 0;
                                }
                                KeyCode::Char('m') | KeyCode::Char('M') => {
                                    self.screen = Screen::Menu;
                                }
                                _ => {}
                            }
                        }

                        Screen::Config => {
                            match key.code {
                                KeyCode::Char('m') | KeyCode::Char('M') => {
                                    self.screen = Screen::Menu;
                                }
                                KeyCode::Char('q') => break,
                                KeyCode::Right | KeyCode::Char('l') => self.next_theme(),
                                KeyCode::Left | KeyCode::Char('h') => self.previous_theme(),
                                _ => {}
                            }
                        }

                        Screen::Main => {
                            match key.code {
                                KeyCode::Char('q') => break,
                                KeyCode::Char('c') => self.screen = Screen::Config,
                                KeyCode::Char('m') | KeyCode::Char('M') => self.screen = Screen::Menu,
                                KeyCode::Right | KeyCode::Char('l') => self.next_tab(),
                                KeyCode::Left | KeyCode::Char('h') => self.previous_tab(),
                                _ => {}
                            }
                        }
                        Screen::Menu => {
                            match key.code {
                                KeyCode::Char('1') | KeyCode::Enter => {
                                    self.screen = Screen::Main;
                                    self.tab_index = 0;
                                }
                                KeyCode::Char('2') | KeyCode::Char('c') => self.screen = Screen::Config,
                                KeyCode::Esc => self.screen = Screen::Title,
                                KeyCode::Char('q') => break,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        disable_raw_mode()?;
        ratatui::restore();
        Ok(())
    }   
}

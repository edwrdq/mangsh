pub mod layout;
pub mod panels;

use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::style::{Modifier, Style};
use crate::app::{App, Screen};

use self::layout::split_main;
use self::panels::{draw_hosts, draw_session, draw_title_screen, draw_config_screen, draw_menu_screen, palette};

pub fn draw(frame: &mut Frame, app: &App) {
    let theme = palette(app.theme);
    let backdrop = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(theme.base));
    frame.render_widget(backdrop, frame.area());

    match app.screen {
        Screen::Title => {
            draw_title_screen(frame, theme);
        }
        Screen::Config => {
            draw_config_screen(frame, app, theme);
        }
        Screen::Menu => {
            draw_menu_screen(frame, theme);
        }
        Screen::Main => {
            let chunks = split_main(frame.area());
            let tabs = Tabs::new(app.tabs.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(theme.accent))
                        .title("Navigation")
                        .style(Style::default().bg(theme.surface).fg(theme.text)),
                )
                .select(app.tab_index)
                .style(Style::default().bg(theme.surface).fg(theme.text_muted))
                .highlight_style(
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD)
                )
                .divider(" · ");

            frame.render_widget(tabs, chunks[0]);

            match app.tab_index {
                0 => draw_hosts(frame, chunks[1], theme),
                1 => draw_session(frame, chunks[1], theme),
                _ => draw_hosts(frame, chunks[1], theme),
            }
        }
    }
}

use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::App,
    theme::{all_themes, Theme},
};

#[derive(Clone, Copy)]
pub struct Palette {
    pub base: Color,
    pub base_alt: Color,
    pub surface: Color,
    pub overlay: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub text: Color,
    pub text_muted: Color,
}

pub fn palette(theme: Theme) -> Palette {
    match theme {
        Theme::Mango => Palette {
            base: Color::Rgb(12, 10, 8),
            base_alt: Color::Rgb(20, 16, 12),
            surface: Color::Rgb(33, 26, 18),
            overlay: Color::Rgb(48, 36, 26),
            accent: Color::Rgb(255, 184, 94),
            accent_soft: Color::Rgb(255, 210, 142),
            text: Color::Rgb(255, 244, 228),
            text_muted: Color::Rgb(214, 183, 150),
        },
        Theme::Forest => Palette {
            base: Color::Rgb(9, 14, 10),
            base_alt: Color::Rgb(16, 24, 18),
            surface: Color::Rgb(26, 38, 29),
            overlay: Color::Rgb(36, 52, 40),
            accent: Color::Rgb(120, 199, 124),
            accent_soft: Color::Rgb(171, 230, 176),
            text: Color::Rgb(226, 241, 229),
            text_muted: Color::Rgb(170, 193, 176),
        },
        Theme::GruvboxDark => Palette {
            base: Color::Rgb(40, 40, 40),
            base_alt: Color::Rgb(50, 48, 47),
            surface: Color::Rgb(60, 56, 54),
            overlay: Color::Rgb(80, 73, 69),
            accent: Color::Rgb(250, 189, 47),
            accent_soft: Color::Rgb(214, 93, 14),
            text: Color::Rgb(235, 219, 178),
            text_muted: Color::Rgb(189, 174, 147),
        },
        Theme::GruvboxLight => Palette {
            base: Color::Rgb(249, 245, 215),
            base_alt: Color::Rgb(235, 219, 178),
            surface: Color::Rgb(221, 214, 171),
            overlay: Color::Rgb(204, 194, 149),
            accent: Color::Rgb(204, 36, 29),
            accent_soft: Color::Rgb(214, 93, 14),
            text: Color::Rgb(60, 56, 54),
            text_muted: Color::Rgb(102, 92, 84),
        },
        Theme::Catppuccin => Palette {
            base: Color::Rgb(24, 25, 38),
            base_alt: Color::Rgb(30, 32, 48),
            surface: Color::Rgb(36, 39, 58),
            overlay: Color::Rgb(46, 50, 70),
            accent: Color::Rgb(137, 180, 250),
            accent_soft: Color::Rgb(245, 189, 230),
            text: Color::Rgb(202, 211, 245),
            text_muted: Color::Rgb(165, 173, 206),
        },
        Theme::Light => Palette {
            base: Color::Rgb(245, 245, 245),
            base_alt: Color::Rgb(235, 235, 235),
            surface: Color::Rgb(225, 229, 234),
            overlay: Color::Rgb(208, 214, 223),
            accent: Color::Rgb(52, 120, 246),
            accent_soft: Color::Rgb(129, 161, 255),
            text: Color::Rgb(35, 38, 47),
            text_muted: Color::Rgb(92, 99, 112),
        },
        Theme::Dark => Palette {
            base: Color::Rgb(12, 12, 14),
            base_alt: Color::Rgb(20, 20, 24),
            surface: Color::Rgb(28, 28, 36),
            overlay: Color::Rgb(40, 40, 50),
            accent: Color::Rgb(96, 165, 250),
            accent_soft: Color::Rgb(167, 139, 250),
            text: Color::Rgb(229, 231, 235),
            text_muted: Color::Rgb(156, 163, 175),
        },
        Theme::Matrix => Palette {
            base: Color::Rgb(0, 0, 0),
            base_alt: Color::Rgb(8, 12, 8),
            surface: Color::Rgb(12, 18, 12),
            overlay: Color::Rgb(16, 24, 16),
            accent: Color::Rgb(0, 255, 90),
            accent_soft: Color::Rgb(120, 255, 160),
            text: Color::Rgb(180, 255, 200),
            text_muted: Color::Rgb(80, 140, 90),
        },
        Theme::Night => Palette {
            base: Color::Rgb(8, 10, 18),
            base_alt: Color::Rgb(14, 17, 28),
            surface: Color::Rgb(22, 27, 44),
            overlay: Color::Rgb(32, 39, 60),
            accent: Color::Rgb(129, 161, 255),
            accent_soft: Color::Rgb(170, 193, 255),
            text: Color::Rgb(228, 234, 255),
            text_muted: Color::Rgb(177, 187, 211),
        },
    }
}

//
// TITLE SCREEN
//
pub fn draw_title_screen(frame: &mut Frame, palette: Palette) {
    let area = frame.area();

    let backdrop = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(palette.base));
    frame.render_widget(backdrop, area);

    let columns = Layout::horizontal([
        Constraint::Percentage(55),
        Constraint::Percentage(45),
    ])
    .margin(2)
    .split(area);

    let header_band = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(palette.accent));
    frame.render_widget(header_band, Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 3,
    });

    let left = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(4),
        Constraint::Fill(1),
    ])
    .split(columns[0]);

    let title_lines = vec![
        Line::from("  __  __    _    _   _   ____  _   _  "),
        Line::from(" |  \\/  |  / \\  | \\ | | / ___|| | | | "),
        Line::from(" | |\\/| | / _ \\ |  \\| | \\___ \\| |_| | "),
        Line::from(" | |  | |/ ___ \\| |\\  |  ___) |  _  | "),
        Line::from(" |_|  |_/_/   \\_\\_| \\_| |____/|_| |_| "),
    ];

    let title = Paragraph::new(Text::from(title_lines))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(palette.accent)
                .bg(palette.base_alt)
                .add_modifier(Modifier::BOLD),
        );

    let tagline = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "A warm mango shell for curating your SSH orchards.",
            Style::default()
                .fg(palette.text)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(Span::styled(
            "Lean, friendly, and ready for late-night sessions.",
            Style::default().fg(palette.text_muted),
        )),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(palette.surface)),
    );

    frame.render_widget(title, left[0]);
    frame.render_widget(tagline, left[1]);

    let right = Layout::vertical([
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(5),
    ])
    .split(columns[1]);

    let badge = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "mangsh",
            Style::default()
                .fg(palette.base)
                .bg(palette.accent_soft)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Terminal SSH companion",
            Style::default().fg(palette.base),
        )),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .bg(palette.accent_soft)
                    .fg(palette.base)
                    .add_modifier(Modifier::BOLD),
            ),
    );

    frame.render_widget(badge, right[0]);

    let hint_box = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "Enter → jump in",
            Style::default().fg(palette.text),
        )),
        Line::from(Span::styled(
            "M → quick menu    q → quit",
            Style::default().fg(palette.text_muted),
        )),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.accent))
            .style(Style::default().bg(palette.surface)),
    );

    frame.render_widget(hint_box, right[2]);
}

//
// MAIN MENU SCREEN
//
pub fn draw_menu_screen(frame: &mut Frame, palette: Palette) {
    let area = frame.area();

    let backdrop = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(palette.base));
    frame.render_widget(backdrop, area);

    let inner = Layout::vertical([
        Constraint::Length(8),
        Constraint::Length(7),
        Constraint::Fill(1),
    ])
    .margin(2)
    .split(area);

    let title_lines = vec![
        Line::from("  __  __    _    _   _   ____  _   _ "),
        Line::from(" |  \\/  |  / \\  | \\ | | / ___|| | | |"),
        Line::from(" | |\\/| | / _ \\ |  \\| | \\___ \\| |_| |"),
        Line::from(" | |  | |/ ___ \\| |\\  |  ___) |  _  |"),
        Line::from(" |_|  |_/_/   \\_\\_| \\_| |____/|_| |_|"),
    ];

    let title = Paragraph::new(Text::from(title_lines))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(palette.accent)
                .bg(palette.base_alt)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(title, inner[0]);

    let options = vec![
        Line::from(Span::styled("[1] Hosts & Session", Style::default().fg(palette.text))),
        Line::from(Span::styled("[2] Config", Style::default().fg(palette.text))),
        Line::from(Span::styled("[q] Quit   [Esc] Title", Style::default().fg(palette.text_muted))),
    ];

    let options_para = Paragraph::new(Text::from(options))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(palette.accent))
                .style(Style::default().bg(palette.surface))
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(palette.text));

    frame.render_widget(options_para, inner[1]);

    let hint = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "Navigate with arrows · M to toggle menu",
            Style::default()
                .fg(palette.text)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Built for a calm, focused terminal session.",
            Style::default()
                .fg(palette.text_muted)
                .add_modifier(Modifier::ITALIC),
        )),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(palette.overlay)),
    );

    frame.render_widget(hint, inner[2]);
}


//
// CONFIG SCREEN
//
pub fn draw_config_screen(frame: &mut Frame, app: &App, palette: Palette) {
    let area = frame.area();

    let body = Block::default()
        .title("Config")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.accent))
        .style(Style::default().bg(palette.surface).fg(palette.text));

    frame.render_widget(body.clone(), area);

    let inner = body.inner(area);
    let sections = Layout::vertical([
        Constraint::Length(6),
        Constraint::Fill(1),
    ])
    .margin(1)
    .split(inner);

    let source = app
        .config_path
        .as_ref()
        .map(|p| format!("Config file: {}", p.display()))
        .unwrap_or_else(|| "Config file: not found (using defaults)".to_string());
    let origin = if app.config.theme.is_some() {
        "Source: configured theme"
    } else {
        "Source: default theme"
    };
    let status = app
        .config_error
        .as_deref()
        .unwrap_or("Loaded successfully.");
    let theme_names = all_themes()
        .iter()
        .map(|t| t.label())
        .collect::<Vec<_>>()
        .join(" · ");

    let theme_box = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            format!("Theme: {}", app.theme.label()),
            Style::default()
                .fg(palette.base)
                .bg(palette.accent_soft)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            source,
            Style::default().fg(palette.text),
        )),
        Line::from(Span::styled(
            origin,
            Style::default().fg(palette.text_muted),
        )),
        Line::from(Span::styled(
            status,
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Use ←/→ to switch theme (auto-saves).",
            Style::default().fg(palette.text),
        )),
        Line::from(Span::styled(
            format!("Available: {theme_names}"),
            Style::default().fg(palette.text_muted),
        )),
    ]))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.accent))
            .style(Style::default().bg(palette.surface).fg(palette.text)),
    );

    frame.render_widget(theme_box, sections[0]);

    let copy = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "Planned: themes, host profiles, SSH keys, Tor mode.",
            Style::default().fg(palette.text),
        )),
        Line::from(Span::styled(
            "Press M for menu or q to exit.",
            Style::default().fg(palette.text_muted),
        )),
    ]))
    .alignment(Alignment::Left)
    .style(Style::default().bg(palette.surface).fg(palette.text_muted));

    frame.render_widget(copy, sections[1]);
}

//
// LEFT PANE: HOSTS
//
pub fn draw_hosts(frame: &mut Frame, area: Rect, palette: Palette) {

    let block = Block::default()
        .title("Hosts")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.accent))
        .style(Style::default().bg(palette.surface).fg(palette.text));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
    let content = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "No hosts added yet.",
            Style::default()
                .fg(palette.text)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Add host profiles in config to see them here.",
            Style::default().fg(palette.text_muted),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Tip: press c to jump into Config.",
            Style::default().fg(palette.accent),
        )),
    ]))
    .alignment(Alignment::Left)
    .style(Style::default().bg(palette.surface).fg(palette.text));

    frame.render_widget(content, inner);
}

//
// RIGHT PANE: SESSION
//
pub fn draw_session(frame: &mut Frame, area: Rect, palette: Palette) {

    let block = Block::default()
        .title("Session")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.accent_soft))
        .style(Style::default().bg(palette.overlay).fg(palette.text));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
    let content = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "Session preview",
            Style::default()
                .fg(palette.accent_soft)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Once connected, live SSH output will appear here.",
            Style::default().fg(palette.text_muted),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Shortcuts",
            Style::default().fg(palette.text),
        )),
        Line::from(Span::styled(
            "←/→ to switch tabs · M for menu · q to quit",
            Style::default().fg(palette.text_muted),
        )),
    ]))
    .alignment(Alignment::Left)
    .style(Style::default().bg(palette.overlay).fg(palette.text));

    frame.render_widget(content, inner);
}

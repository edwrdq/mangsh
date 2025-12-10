use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Mango,
    #[serde(alias = "gruvbox", alias = "gruvbox-dark")]
    GruvboxDark,
    GruvboxLight,
    #[serde(alias = "catpuccin")]
    Catppuccin,
    Light,
    Dark,
    Matrix,
    Forest,
    Night,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Mango
    }
}

pub fn all_themes() -> &'static [Theme] {
    &[
        Theme::Mango,
        Theme::Forest,
        Theme::Night,
        Theme::GruvboxDark,
        Theme::GruvboxLight,
        Theme::Catppuccin,
        Theme::Light,
        Theme::Dark,
        Theme::Matrix,
    ]
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Mango => "Mango",
            Theme::Forest => "Forest",
            Theme::Night => "Night",
            Theme::GruvboxDark => "Gruvbox Dark",
            Theme::GruvboxLight => "Gruvbox Light",
            Theme::Catppuccin => "Catppuccin",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Matrix => "Matrix",
        }
    }
}

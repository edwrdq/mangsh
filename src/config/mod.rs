pub mod loader;
pub mod schema;

use serde::{Deserialize, Serialize};
use crate::theme::Theme;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub theme: Option<Theme>,
}

impl Config {
    pub fn effective_theme(&self) -> Theme {
        self.theme.unwrap_or_default()
    }
}

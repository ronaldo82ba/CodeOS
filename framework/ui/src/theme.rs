use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: u32,
    pub background: u32,
    pub surface: u32,
    pub on_surface: u32,
}

pub struct CodeOsTheme;

impl CodeOsTheme {
    pub fn dark() -> ThemeColors {
        ThemeColors {
            primary: 0x0066FF,
            background: 0x0A0A0F,
            surface: 0x16161E,
            on_surface: 0xF0F0F5,
        }
    }

    pub fn light() -> ThemeColors {
        ThemeColors {
            primary: 0x0052CC,
            background: 0xF5F5FA,
            surface: 0xFFFFFF,
            on_surface: 0x1A1A24,
        }
    }
}

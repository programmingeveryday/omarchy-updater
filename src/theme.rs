use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct OmarchyColors {
    pub mode: Option<String>,
    pub accent: Option<String>,
    pub selection: Option<String>,
    pub muted: Option<String>,
    pub background: Option<String>,
    pub dark_background: Option<String>,
    pub darker_background: Option<String>,
    pub lighter_background: Option<String>,
    pub foreground: Option<String>,
    pub dark_foreground: Option<String>,
    pub light_foreground: Option<String>,
    pub bright_foreground: Option<String>,
    pub red: Option<String>,
    pub yellow: Option<String>,
    pub green: Option<String>,
    pub cyan: Option<String>,
    pub blue: Option<String>,
    pub magenta: Option<String>,
}

impl OmarchyColors {
    pub fn load_current() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/edward".to_string());
        let path = PathBuf::from(&home).join(".local/state/omarchy/current/theme/colors.toml");
        
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(colors) = toml::from_str::<OmarchyColors>(&content) {
                return colors;
            }
        }
        
        // Fallback default
        Self {
            mode: Some("light".to_string()),
            accent: Some("#316ac5".to_string()),
            background: Some("#eaf4ff".to_string()),
            foreground: Some("#173d75".to_string()),
            lighter_background: Some("#ffffff".to_string()),
            green: Some("#228b22".to_string()),
            red: Some("#b23b35".to_string()),
            yellow: Some("#9b6700".to_string()),
            muted: Some("#8aaee0".to_string()),
            ..Default::default()
        }
    }

    pub fn to_gtk_css(&self) -> String {
        let bg = self.background.as_deref().unwrap_or("#eaf4ff");
        let card_bg = self.lighter_background.as_deref().unwrap_or("#ffffff");
        let fg = self.foreground.as_deref().unwrap_or("#173d75");
        let accent = self.accent.as_deref().unwrap_or("#316ac5");
        let muted = self.muted.as_deref().unwrap_or("#8aaee0");
        let green = self.green.as_deref().unwrap_or("#38a169");
        let red = self.red.as_deref().unwrap_or("#e53e3e");

        format!(
            r#"
            window.omarchy-updater {{
                background-color: {bg};
                color: {fg};
            }}

            .main-header {{
                font-weight: 800;
                font-size: 20px;
                color: {fg};
            }}

            .badge-up-to-date {{
                background-color: {green};
                color: #ffffff;
                border-radius: 12px;
                padding: 4px 12px;
                font-weight: 700;
                font-size: 13px;
            }}

            .badge-updates-available {{
                background-color: {accent};
                color: #ffffff;
                border-radius: 12px;
                padding: 4px 12px;
                font-weight: 700;
                font-size: 13px;
            }}

            .update-card {{
                background-color: {card_bg};
                border-radius: 12px;
                border: 1px solid {muted};
                padding: 16px;
                margin-bottom: 8px;
            }}

            .btn-update {{
                background: {accent};
                color: #ffffff;
                font-weight: 700;
                font-size: 15px;
                padding: 10px 24px;
                border-radius: 24px;
                border: none;
            }}

            .btn-update:hover {{
                opacity: 0.9;
            }}

            .btn-refresh {{
                background: transparent;
                border-radius: 24px;
                border: 1px solid {muted};
                color: {fg};
                padding: 8px 16px;
            }}

            .log-terminal {{
                background-color: #11111b;
                color: #a6e3a1;
                font-family: monospace;
                font-size: 12px;
                border-radius: 8px;
                padding: 12px;
            }}
            "#
        )
    }
}

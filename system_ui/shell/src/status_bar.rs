use codeos_ui::ThemeColors;

pub fn render(theme: &ThemeColors) {
    tracing::debug!(
        primary = format!("#{:06X}", theme.primary),
        "status bar rendered"
    );
}

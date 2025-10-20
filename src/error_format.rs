use anstyle::{AnsiColor, Color, Style};
use std::io::IsTerminal;

/// Determines if colored output should be used based on environment
fn should_use_color_with_env(no_color: bool, force_color: bool, is_terminal: bool) -> bool {
    if force_color {
        return true;
    }
    if no_color {
        return false;
    }
    is_terminal
}

/// Determines if colored output should be used
fn should_use_color() -> bool {
    should_use_color_with_env(
        std::env::var("NO_COLOR").is_ok(),
        std::env::var("CLICOLOR_FORCE").is_ok() || std::env::var("FORCE_COLOR").is_ok(),
        std::io::stderr().is_terminal(),
    )
}

/// Format an error message with red color and ❌ emoji
pub fn format_error(msg: &str) -> String {
    if should_use_color() {
        let error_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red)));
        format!(
            "{}❌ {}{}",
            error_style.render(),
            msg,
            error_style.render_reset()
        )
    } else {
        format!("❌ {}", msg)
    }
}

/// Format a warning message with yellow color and 🟡 emoji
pub fn format_warning(msg: &str) -> String {
    if should_use_color() {
        let warning_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));
        format!(
            "{}🟡 {}{}",
            warning_style.render(),
            msg,
            warning_style.render_reset()
        )
    } else {
        format!("🟡 {}", msg)
    }
}

/// Format a hint message with dim color and 💡 emoji
pub fn format_hint(msg: &str) -> String {
    if should_use_color() {
        let hint_style = Style::new().dimmed();
        format!(
            "{}💡 {}{}",
            hint_style.render(),
            msg,
            hint_style.render_reset()
        )
    } else {
        format!("💡 {}", msg)
    }
}

/// Format text with bold styling
pub fn bold(text: &str) -> String {
    if should_use_color() {
        let bold_style = Style::new().bold();
        format!(
            "{}{}{}",
            bold_style.render(),
            text,
            bold_style.render_reset()
        )
    } else {
        text.to_string()
    }
}

/// Format an error message with bold emphasis on specific parts
///
/// Example: `format_error_with_bold("Branch '", "feature-x", "' already exists")`
pub fn format_error_with_bold(prefix: &str, emphasized: &str, suffix: &str) -> String {
    if should_use_color() {
        let error_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red)));
        let bold_style = Style::new()
            .fg_color(Some(Color::Ansi(AnsiColor::Red)))
            .bold();
        format!(
            "{}❌ {}{}{}{}{}{}",
            error_style.render(),
            prefix,
            bold_style.render(),
            emphasized,
            error_style.render(), // Back to regular red
            suffix,
            error_style.render_reset()
        )
    } else {
        format!("❌ {}{}{}", prefix, emphasized, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_use_color_force_color() {
        assert!(should_use_color_with_env(false, true, false));
        assert!(should_use_color_with_env(true, true, false));
    }

    #[test]
    fn test_should_use_color_no_color() {
        assert!(!should_use_color_with_env(true, false, true));
        assert!(!should_use_color_with_env(true, false, false));
    }

    #[test]
    fn test_should_use_color_terminal() {
        assert!(should_use_color_with_env(false, false, true));
        assert!(!should_use_color_with_env(false, false, false));
    }
}

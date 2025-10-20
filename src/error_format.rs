use anstyle::{AnsiColor, Color, Style};
use std::io::IsTerminal;

/// Determines if colored output should be used
fn should_use_color() -> bool {
    // Check for force color environment variables (for testing)
    if std::env::var("CLICOLOR_FORCE").is_ok() || std::env::var("FORCE_COLOR").is_ok() {
        return true;
    }

    // Check if NO_COLOR is set (universal way to disable colors)
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Otherwise use TTY detection
    std::io::stderr().is_terminal()
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

/// Format text with a specific color
pub fn colored(text: &str, color: AnsiColor) -> String {
    if should_use_color() {
        let style = Style::new().fg_color(Some(Color::Ansi(color)));
        format!("{}{}{}", style.render(), text, style.render_reset())
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
    fn test_format_error_no_color() {
        // Ensure NO_COLOR disables formatting
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
        let result = format_error("Test error");
        assert_eq!(result, "❌ Test error");
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn test_format_error_with_bold_no_color() {
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
        let result = format_error_with_bold("Branch '", "main", "' already exists");
        assert_eq!(result, "❌ Branch 'main' already exists");
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn test_format_warning() {
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
        let result = format_warning("Test warning");
        assert_eq!(result, "🟡 Test warning");
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn test_format_hint() {
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
        let result = format_hint("Test hint");
        assert_eq!(result, "💡 Test hint");
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn test_bold() {
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }
        let result = bold("important");
        assert_eq!(result, "important");
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }
}

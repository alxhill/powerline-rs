use std::sync::OnceLock;

use crate::colors::Color;

pub static SHELL: OnceLock<Shell> = OnceLock::new();

#[derive(Debug)]
pub enum Shell {
    Bash,
    Bare,
    Zsh,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BgColor(u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FgColor(u8);

pub struct Reset;

impl FgColor {
    pub fn transpose(self) -> BgColor {
        BgColor(self.0)
    }
}

/// The OSC 8 hyperlink open (`ESC ]8;;<url> ST`) and close (`ESC ]8;; ST`)
/// sequences for the active shell, wrapped so the line editor treats them as
/// zero-width.
///
/// `BgColor`/`FgColor`/`Reset` already wrap themselves, but the hyperlink built
/// in `add_hyperlink_segment` bypasses them. Without the markers, bash readline
/// and zsh count the escapes - and the whole URL - as visible columns, which
/// corrupts cursor tracking so the prompt smears on redraw (fish/pwsh handle
/// raw escapes themselves and need no markers).
///
/// Bash needs more than wrapping: it must use readline's `\e` spelling for ESC
/// (a raw ESC works for the terminal but the project escapes everything as
/// `\e`), and the literal backslash of the ST terminator must be `\\` - a raw
/// `\` sitting right before the `\]` close marker would be decoded as the pair
/// `\\` (a literal backslash), leaving the `\[` region unterminated.
pub fn hyperlink(url: &str) -> (String, String) {
    hyperlink_for(SHELL.get().expect("shell not specified!"), url)
}

/// Pure resolver behind [`hyperlink`], split out so the per-shell escaping can
/// be tested without the process-global `SHELL` (which is set only once).
fn hyperlink_for(shell: &Shell, url: &str) -> (String, String) {
    match shell {
        Shell::Bash => (
            format!(r"\[\e]8;;{url}\e\\\]"),
            r"\[\e]8;;\e\\\]".to_string(),
        ),
        Shell::Bare => (format!("\x1b]8;;{url}\x1b\\"), "\x1b]8;;\x1b\\".to_string()),
        Shell::Zsh => (
            format!("%{{\x1b]8;;{url}\x1b\\%}}"),
            "%{\x1b]8;;\x1b\\%}".to_string(),
        ),
    }
}

impl From<Color> for FgColor {
    fn from(c: Color) -> Self {
        FgColor(c.0)
    }
}

impl BgColor {
    pub fn transpose(self) -> FgColor {
        FgColor(self.0)
    }
}

impl From<Color> for BgColor {
    fn from(c: Color) -> Self {
        BgColor(c.0)
    }
}

impl std::fmt::Display for BgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match SHELL.get().expect("shell not specified!") {
            Shell::Bash => write!(f, r#"\[\e[48;5;{}m\]"#, self.0),
            Shell::Bare => write!(f, "\x1b[48;5;{}m", self.0),
            Shell::Zsh => write!(f, "%{{\x1b[48;5;{}m%}}", self.0),
        }
    }
}

impl std::fmt::Display for FgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match SHELL.get().expect("shell not specified!") {
            Shell::Bash => write!(f, r#"\[\e[38;5;{}m\]"#, self.0),
            Shell::Bare => write!(f, "\x1b[38;5;{}m", self.0),
            Shell::Zsh => write!(f, "%{{\x1b[38;5;{}m%}}", self.0),
        }
    }
}

impl std::fmt::Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match SHELL.get().expect("shell not specified!") {
            Shell::Bash => f.write_str(r#"\[\e[0m\]"#),
            Shell::Bare => f.write_str("\x1b[0m"),
            Shell::Zsh => f.write_str("%{\x1b[39m%}%{\x1b[49m%}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const URL: &str = "https://example.com/pr/1";

    #[test]
    fn bash_hyperlink_uses_readline_markers_and_no_raw_escapes() {
        let (open, close) = hyperlink_for(&Shell::Bash, URL);
        // The non-printing markers must balance, ESC must be spelled `\e` (no
        // raw byte), and the ST's backslash must be `\\` so it doesn't merge
        // with the `\]` close marker and leave the region unterminated.
        assert_eq!(open, r"\[\e]8;;https://example.com/pr/1\e\\\]");
        assert_eq!(close, r"\[\e]8;;\e\\\]");
        for s in [&open, &close] {
            assert!(s.starts_with(r"\[") && s.ends_with(r"\]"));
            assert!(!s.contains('\x1b'), "bash must not emit a raw ESC byte");
        }
    }

    #[test]
    fn zsh_hyperlink_wraps_raw_escapes_in_percent_markers() {
        let (open, close) = hyperlink_for(&Shell::Zsh, URL);
        assert_eq!(open, "%{\x1b]8;;https://example.com/pr/1\x1b\\%}");
        assert_eq!(close, "%{\x1b]8;;\x1b\\%}");
    }

    #[test]
    fn bare_hyperlink_is_unwrapped_raw_osc8() {
        let (open, close) = hyperlink_for(&Shell::Bare, URL);
        assert_eq!(open, "\x1b]8;;https://example.com/pr/1\x1b\\");
        assert_eq!(close, "\x1b]8;;\x1b\\");
    }
}

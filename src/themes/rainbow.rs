use crate::colors::*;
use crate::colors::Color;
use crate::modules::{
    CargoScheme, CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme,
    LastCmdDurationScheme, PythonEnvScheme, ReadOnlyScheme, SpacerScheme, TimeScheme, UserScheme,
};
use crate::themes::{CompleteTheme, DefaultColors};

#[derive(Copy, Clone)]
pub struct RainbowTheme;

impl DefaultColors for RainbowTheme {
    fn default_bg() -> Color {
        blue()
    }

    fn default_fg() -> Color {
        light_grey()
    }
}

impl CompleteTheme for RainbowTheme {}

impl TimeScheme for RainbowTheme {
    const TIME_BG: Color = dark_grey();
    const TIME_FG: Color = mid_grey();
}

impl CargoScheme for RainbowTheme {
    fn cargo_fg() -> Color {
        dark_grey()
    }

    fn cargo_bg() -> Color {
        burnt_orange()
    }
}

impl UserScheme for RainbowTheme {
    const USERNAME_ROOT_BG: Color = red();
    const USERNAME_BG: Color = black();
    const USERNAME_FG: Color = green();
}

impl HostScheme for RainbowTheme {
    const HOSTNAME_FG: Color = grey();
    const HOSTNAME_BG: Color = dark_grey();
}

impl ExitCodeScheme for RainbowTheme {
    const EXIT_CODE_BG: Color = red();
    const EXIT_CODE_FG: Color = white();
}

impl CmdScheme for RainbowTheme {
    fn cmd_passed_fg() -> Color {
        green()
    }

    fn cmd_passed_bg() -> Color {
        black()
    }

    fn cmd_failed_bg() -> Color {
        warning_red()
    }

    fn cmd_failed_fg() -> Color {
        white()
    }

    fn cmd_user_symbol() -> &'static str {
        "\u{f105}"
    }
}

impl CwdScheme for RainbowTheme {
    fn path_bg_colors() -> Vec<Color> {
        vec![red(), orange(), yellow(), green(), blue(), nice_puple()]
    }
}

impl GitScheme for RainbowTheme {
    const GIT_REMOTE_BG: Color = mid_grey();
    const GIT_REMOTE_FG: Color = light_grey();
    const GIT_STAGED_BG: Color = forest_green();
    const GIT_STAGED_FG: Color = white();
    const GIT_NOTSTAGED_BG: Color = mid_red();
    const GIT_NOTSTAGED_FG: Color = white();
    const GIT_UNTRACKED_BG: Color = warning_red();
    const GIT_UNTRACKED_FG: Color = white();
    const GIT_CONFLICTED_BG: Color = light_red();
    const GIT_CONFLICTED_FG: Color = white();
    const GIT_REPO_CLEAN_BG: Color = blue();
    const GIT_REPO_CLEAN_FG: Color = white();
    const GIT_REPO_DIRTY_BG: Color = bright_orange();
    const GIT_REPO_DIRTY_FG: Color = white();
    const NOT_STAGED_SYMBOL: &'static str = "\u{eae9}"; // pencil
    const STAGED_SYMBOL: &'static str = "+";
    const UNTRACKED_SYMBOL: &'static str = "?";
}

impl ReadOnlyScheme for RainbowTheme {
    const READONLY_FG: Color = Color(254);
    const READONLY_BG: Color = Color(124);
    const READONLY_SYMBOL: &'static str = "\u{f0221}";
}

impl PythonEnvScheme for RainbowTheme {
    fn pyenv_fg() -> Color {
        dark_grey()
    }

    fn pyenv_bg() -> Color {
        light_green()
    }

    fn pyver_fg() -> Color {
        dark_grey()
    }

    fn pyver_bg() -> Color {
        mid_green()
    }
}

impl SpacerScheme for RainbowTheme {}

impl LastCmdDurationScheme for RainbowTheme {
    fn time_bg() -> Color {
        black()
    }

    fn time_fg() -> Color {
        green()
    }

    fn time_icon() -> &'static str {
        ""
    }
}

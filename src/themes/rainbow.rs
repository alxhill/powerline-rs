use crate::colors::*;
use crate::modules::{
    CargoScheme, CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme,
    LastCmdDurationScheme, PythonEnvScheme, ReadOnlyScheme, SpacerScheme, TimeScheme, UserScheme,
};
use crate::themes::CompleteTheme;
use crate::Color;

#[derive(Copy, Clone)]
pub struct RainbowTheme;

impl CompleteTheme for RainbowTheme {}

impl TimeScheme for RainbowTheme {
    const TIME_BG: Color = dark_grey();
    const TIME_FG: Color = mid_grey();
}

impl CargoScheme for RainbowTheme {
    const CARGO_BG: Color = burnt_orange();
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
    const CMD_PASSED_FG: Color = green();
    const CMD_PASSED_BG: Color = black();
    const CMD_FAILED_BG: Color = warning_red();
    const CMD_FAILED_FG: Color = white();
    const CMD_USER_SYMBOL: &'static str = "\u{f105}";
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
    const PYVENV_FG: Color = dark_grey();
    const PYVENV_BG: Color = light_green();
    const PYVER_FG: Color = dark_grey();
    const PYVER_BG: Color = mid_green();
}

impl SpacerScheme for RainbowTheme {}

impl LastCmdDurationScheme for RainbowTheme {
    const TIME_BG: Color = black();
    const TIME_FG: Color = green();
    const TIME_ICON: &'static str = "";
}

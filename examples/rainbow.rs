use chrono::Duration;
use powerline::modules::*;
use powerline::powerline::Separator;
use powerline::{colors, Color};
use std::env;

#[derive(Copy, Clone)]
pub struct RainbowTheme;

impl CmdScheme for RainbowTheme {
    const CMD_PASSED_FG: Color = colors::green();
    const CMD_PASSED_BG: Color = colors::black();
    const CMD_FAILED_BG: Color = colors::warning_red();
    const CMD_FAILED_FG: Color = colors::light_grey();
    const CMD_USER_SYMBOL: &'static str = "\u{f105}";
}

impl CwdScheme for RainbowTheme {
    const PATH_FG: Color = colors::light_grey();
    const PATH_BG: Color = Color(236);
}

impl GitScheme for RainbowTheme {
    const GIT_AHEAD_BG: Color = Color(240);
    const GIT_AHEAD_FG: Color = Color(250);
    const GIT_BEHIND_BG: Color = Color(240);
    const GIT_BEHIND_FG: Color = Color(250);
    const GIT_STAGED_BG: Color = Color(22);
    const GIT_STAGED_FG: Color = colors::light_grey();
    const GIT_NOTSTAGED_BG: Color = colors::warning_red();
    const GIT_NOTSTAGED_FG: Color = colors::light_grey();
    const GIT_UNTRACKED_BG: Color = Color(52);
    const GIT_UNTRACKED_FG: Color = colors::light_grey();
    const GIT_CONFLICTED_BG: Color = Color(9);
    const GIT_CONFLICTED_FG: Color = colors::light_grey();
    const GIT_REPO_CLEAN_BG: Color = colors::blue();
    const GIT_REPO_CLEAN_FG: Color = colors::light_grey();
    const GIT_REPO_DIRTY_BG: Color = Color(202);
    const GIT_REPO_DIRTY_FG: Color = colors::light_grey();
    const NOT_STAGED_SYMBOL: &'static str = "\u{f0deb}"; // pencil with +
    const STAGED_SYMBOL: &'static str = "\u{f067}"; // plus
    const UNTRACKED_SYMBOL: &'static str = "\u{f086f}"; // file with ?
}

impl ReadOnlyScheme for RainbowTheme {
    const READONLY_FG: Color = Color(254);
    const READONLY_BG: Color = Color(124);
}

impl PythonEnvScheme for RainbowTheme {
    const SEPARATOR: Separator = Separator::ChevronLeft;
    const PYVENV_FG: Color = Color(0);
    const PYVENV_BG: Color = Color(42);
}

impl SpacerScheme for RainbowTheme {}

impl LastCmdDurationScheme for RainbowTheme {
    const TIME_BG: Color = colors::black();
    const TIME_FG: Color = colors::green();
    const TIME_ICON: &'static str = "";
}

fn main() {
    let top_prompt = powerline::Powerline::new()
        .set_separator(Separator::ChevronRight)
        .add_module(Spacer::<RainbowTheme>::small())
        .add_module(Cwd::<RainbowTheme>::new(45, 4, false))
        .add_module(ReadOnly::<RainbowTheme>::new())
        .add_module(Spacer::<RainbowTheme>::large())
        .set_separator(Separator::RoundRight)
        .add_module(Git::<RainbowTheme>::new());

    let mini_prompt = powerline::Powerline::new()
        .add_module(LastCmdDuration::<RainbowTheme>::new(
            Duration::milliseconds(50),
        ))
        .add_module(Cmd::<RainbowTheme>::new());

    println!("{}", top_prompt);
    print!("{} ", mini_prompt);
}

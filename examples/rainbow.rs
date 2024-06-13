use chrono::Duration;
use powerline::modules::*;
use powerline::powerline::Separator;
use powerline::{colors::*, Color};
use std::env;

#[derive(Copy, Clone)]
pub struct RainbowTheme;

impl CmdScheme for RainbowTheme {
    const CMD_PASSED_FG: Color = green();
    const CMD_PASSED_BG: Color = black();
    const CMD_FAILED_BG: Color = warning_red();
    const CMD_FAILED_FG: Color = white();
    const CMD_USER_SYMBOL: &'static str = "\u{f105}";
}

impl CwdScheme for RainbowTheme {}

impl GitScheme for RainbowTheme {
    const GIT_REMOTE_BG: Color = mid_grey();
    const GIT_REMOTE_FG: Color = light_grey();
    const GIT_STAGED_BG: Color = forest_green();
    const GIT_STAGED_FG: Color = white();
    const GIT_NOTSTAGED_BG: Color = warning_red();
    const GIT_NOTSTAGED_FG: Color = white();
    const GIT_UNTRACKED_BG: Color = burgundy();
    const GIT_UNTRACKED_FG: Color = white();
    const GIT_CONFLICTED_BG: Color = light_red();
    const GIT_CONFLICTED_FG: Color = white();
    const GIT_REPO_CLEAN_BG: Color = blue();
    const GIT_REPO_CLEAN_FG: Color = white();
    const GIT_REPO_DIRTY_BG: Color = bright_orange();
    const GIT_REPO_DIRTY_FG: Color = white();
    const NOT_STAGED_SYMBOL: &'static str = "\u{f0deb}"; // pencil with +
    const STAGED_SYMBOL: &'static str = "+"; // plus
    const UNTRACKED_SYMBOL: &'static str = "?"; // file with ?
}

impl ReadOnlyScheme for RainbowTheme {
    const READONLY_FG: Color = Color(254);
    const READONLY_BG: Color = Color(124);
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

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let (status, duration, columns): (&str, &str, &str) = match args.as_slice() {
        [_, status, duration, columns] => (status, duration, columns),
        [_, status, duration] => (status, duration, "0"),
        _ => ("0", "0", "0"),
    };

    let columns = str::parse::<usize>(columns).unwrap_or(0);
    let duration = str::parse::<i64>(duration)
        .map(Duration::milliseconds)
        .unwrap_or(Duration::seconds(0));

    let top_prompt = powerline::Powerline::new()
        .set_separator(Separator::Chevron)
        .add_module(Spacer::<RainbowTheme>::small())
        .add_module(Cwd::<RainbowTheme>::new(45, 4, false))
        .add_module(ReadOnly::<RainbowTheme>::new())
        // .add_padding(2, None)
        .add_module(Spacer::<RainbowTheme>::small())
        .set_separator(Separator::Chevron)
        .add_module(Git::<RainbowTheme>::new())
        .to_right()
        .set_separator(Separator::Round)
        .add_module(PythonEnv::<RainbowTheme>::new())
        .add_padding(0, None)
        .render(columns);

    let mini_prompt = powerline::Powerline::new()
        .add_module(LastCmdDuration::<RainbowTheme>::new(
            duration,
            Duration::milliseconds(0),
        ))
        .add_module(Cmd::<RainbowTheme>::new(status.to_owned()))
        .render(columns);

    println!("{}", top_prompt);
    print!("{} ", mini_prompt);
}

use powerline;
use powerline::{Color, colors};
use powerline::modules::*;
use powerline::theme::SimpleTheme;

#[derive(Copy, Clone)]
pub struct RainbowTheme;

impl CmdScheme for RainbowTheme {
    const CMD_PASSED_FG: Color = colors::light_grey();
    const CMD_PASSED_BG: Color = colors::green();
    const CMD_FAILED_BG: Color = colors::red();
    const CMD_FAILED_FG: Color = colors::light_grey();
    const CMD_USER_SYMBOL: &'static str = "$";
}

impl CwdScheme for RainbowTheme {
    const PATH_FG: Color = Color(254);
    const PATH_BG: Color = Color(53);
    const HOME_FG: Color = colors::light_grey();
    const HOME_BG: Color = colors::turquoise_blue();
    const SEPARATOR_FG: Color = Color(40);
}

impl ExitCodeScheme for RainbowTheme {
    const EXIT_CODE_BG: Color = colors::mid_red();
    const EXIT_CODE_FG: Color = Color(0);
}

impl GitScheme for RainbowTheme {
    const GIT_AHEAD_BG: Color = Color(240);
    const GIT_AHEAD_FG: Color = Color(250);
    const GIT_BEHIND_BG: Color = Color(240);
    const GIT_BEHIND_FG: Color = Color(250);
    const GIT_STAGED_BG: Color = Color(22);
    const GIT_STAGED_FG: Color = Color(15);
    const GIT_NOTSTAGED_BG: Color = Color(130);
    const GIT_NOTSTAGED_FG: Color = Color(15);
    const GIT_UNTRACKED_BG: Color = Color(52);
    const GIT_UNTRACKED_FG: Color = Color(15);
    const GIT_CONFLICTED_BG: Color = Color(9);
    const GIT_CONFLICTED_FG: Color = Color(15);
    const GIT_REPO_CLEAN_BG: Color = Color(148);
    const GIT_REPO_CLEAN_FG: Color = Color(0);
    const GIT_REPO_DIRTY_BG: Color = Color(161);
    const GIT_REPO_DIRTY_FG: Color = Color(15);
}

impl ReadOnlyScheme for RainbowTheme {
    const READONLY_FG: Color = Color(254);
    const READONLY_BG: Color = Color(124);
}

impl VirtualEnvScheme for RainbowTheme {
    const PYVENV_FG: Color = Color(0);
    const PYVENV_BG: Color = Color(42);
}

fn main() {
    let mut top_prompt = powerline::Powerline::new();

    top_prompt.add_module(Cwd::<RainbowTheme>::new(45, 4, false));
    top_prompt.add_module(Git::<RainbowTheme>::new());
    top_prompt.add_module(ReadOnly::<RainbowTheme>::new());
    top_prompt.add_module(VirtualEnv::<RainbowTheme>::new());

    let mut mini_prompt = powerline::Powerline::new();
    mini_prompt.add_module(ExitCode::<RainbowTheme>::new());
    mini_prompt.add_module(Cmd::<RainbowTheme>::new());

    println!("{}", top_prompt);
    print!("{}", mini_prompt);
}
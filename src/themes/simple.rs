use crate::{Color, colors};
use crate::modules::{CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme, ReadOnlyScheme, TimeScheme, UserScheme};

#[derive(Copy, Clone)]
pub struct SimpleTheme;

impl CmdScheme for SimpleTheme {
    const CMD_PASSED_FG: Color = Color(15);
    const CMD_PASSED_BG: Color = Color(236);
    const CMD_FAILED_BG: Color = Color(161);
    const CMD_FAILED_FG: Color = Color(15);
}

impl CwdScheme<1> for SimpleTheme {
    const PATH_BG_COLORS: [Color; 1] = [
        colors::grey()
    ];
}

impl ExitCodeScheme for SimpleTheme {
    const EXIT_CODE_BG: Color = Color(161);
    const EXIT_CODE_FG: Color = Color(15);
}

impl UserScheme for SimpleTheme {
    const USERNAME_ROOT_BG: Color = Color(124);
    const USERNAME_BG: Color = Color(240);
    const USERNAME_FG: Color = Color(250);
}

impl HostScheme for SimpleTheme {
    const HOSTNAME_FG: Color = Color(250);
    const HOSTNAME_BG: Color = Color(238);
}

impl ReadOnlyScheme for SimpleTheme {
    const READONLY_FG: Color = Color(254);
    const READONLY_BG: Color = Color(124);
}

impl TimeScheme for SimpleTheme {
    const TIME_BG: Color = Color(238);
    const TIME_FG: Color = Color(250);
}

impl GitScheme for SimpleTheme {
    const GIT_REMOTE_BG: Color = Color(240);
    const GIT_REMOTE_FG: Color = Color(250);
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

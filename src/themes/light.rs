use crate::modules::{
    CargoScheme, CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme,
    LastCmdDurationScheme, PythonEnvScheme, ReadOnlyScheme, SpacerScheme, TimeScheme, UserScheme,
};
use crate::themes::CompleteTheme;
use crate::Color;

#[derive(Copy, Clone)]
pub struct LightTheme;

impl CmdScheme for LightTheme {
    const CMD_PASSED_FG: Color = Color(0);
    const CMD_PASSED_BG: Color = Color(0);
    const CMD_FAILED_BG: Color = Color(0);
    const CMD_FAILED_FG: Color = Color(0);
}

impl CwdScheme for LightTheme {
    fn path_bg_colors() -> Vec<Color> {
        todo!()
    }
}

impl LastCmdDurationScheme for LightTheme {
    const TIME_BG: Color = Color(0);
    const TIME_FG: Color = Color(0);
}

impl ExitCodeScheme for LightTheme {
    const EXIT_CODE_BG: Color = Color(0);
    const EXIT_CODE_FG: Color = Color(0);
}

impl GitScheme for LightTheme {
    const GIT_REMOTE_BG: Color = Color(0);
    const GIT_REMOTE_FG: Color = Color(0);
    const GIT_STAGED_BG: Color = Color(0);
    const GIT_STAGED_FG: Color = Color(0);
    const GIT_NOTSTAGED_BG: Color = Color(0);
    const GIT_NOTSTAGED_FG: Color = Color(0);
    const GIT_UNTRACKED_BG: Color = Color(0);
    const GIT_UNTRACKED_FG: Color = Color(0);
    const GIT_CONFLICTED_BG: Color = Color(0);
    const GIT_CONFLICTED_FG: Color = Color(0);
    const GIT_REPO_CLEAN_BG: Color = Color(0);
    const GIT_REPO_CLEAN_FG: Color = Color(0);
    const GIT_REPO_DIRTY_BG: Color = Color(0);
    const GIT_REPO_DIRTY_FG: Color = Color(0);
}

impl PythonEnvScheme for LightTheme {
    const PYVENV_FG: Color = Color(0);
    const PYVENV_BG: Color = Color(0);
    const PYVER_FG: Color = Color(0);
    const PYVER_BG: Color = Color(0);
}

impl ReadOnlyScheme for LightTheme {
    const READONLY_FG: Color = Color(0);
    const READONLY_BG: Color = Color(0);
}

impl SpacerScheme for LightTheme {}

impl HostScheme for LightTheme {
    const HOSTNAME_FG: Color = Color(0);
    const HOSTNAME_BG: Color = Color(0);
}

impl UserScheme for LightTheme {
    const USERNAME_ROOT_BG: Color = Color(0);
    const USERNAME_BG: Color = Color(0);
    const USERNAME_FG: Color = Color(0);
}

impl CargoScheme for LightTheme {
    const CARGO_BG: Color = Color(0);
}

impl TimeScheme for LightTheme {
    const TIME_BG: Color = Color(0);
    const TIME_FG: Color = Color(0);
}

impl CompleteTheme for LightTheme {}

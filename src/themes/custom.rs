use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::colors::Color;
use crate::colors::dark_grey;
use crate::modules::{
    CargoScheme, CmdScheme, CwdScheme, ExitCodeScheme, GitScheme, HostScheme,
    LastCmdDurationScheme, PythonEnvScheme, ReadOnlyScheme, SpacerScheme, TimeScheme, UserScheme,
};
use crate::themes::{CompleteTheme, DefaultColors};

#[derive(Clone)]
pub struct CustomTheme;

static THEME: OnceLock<CustomThemeImpl> = OnceLock::new();

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum ColorsJson {
    Named(String),
    Code(u8),
}

impl From<&ColorsJson> for Color {
    fn from(value: &ColorsJson) -> Self {
        match value {
            ColorsJson::Named(col_name) => {
                let c = col_name.as_str();
                Color::from_name(c).expect("unknown color")
            }
            ColorsJson::Code(col_code) => Color(*col_code),
        }
    }
}

#[derive(Deserialize)]
struct DefaultColorsJson {
    fg: ColorsJson,
    bg: ColorsJson,
    // secondary_fg: Option<ColorsJson>,
    // secondary_bg: Option<ColorsJson>,
    // alert_fg: Option<ColorsJson>,
    // alert_bg: Option<ColorsJson>,
}

#[derive(Deserialize)]
struct CustomThemeImpl {
    defaults: DefaultColorsJson,
    modules: HashMap<String, HashMap<String, ColorsJson>>,
}

impl CustomThemeImpl {
    fn get_color(&self, module: &str, color: &str) -> Option<Color> {
        self.modules
            .get(module)
            .and_then(|colors| colors.get(color))
            .map(|col_json| col_json.into())
    }
}

impl CustomTheme {
    pub fn load(path: PathBuf) {
        let theme: CustomThemeImpl = serde_json::from_reader(File::open(path).unwrap()).unwrap();
        let _ = THEME.set(theme);

        // todo: figure out why this is being set twice...
        // match THEME.set(theme) {
        //     Ok(()) => {
        //         println!("{:?} | finish custom theme load {x}", thread_id);
        //     }
        //     Err(e) => {
        //         println!("{:?} | failed to set custom theme? {:?}, {x}", thread_id, e);
        //     }
        // }
    }

    pub fn get_color(module: &str, color: &str) -> Option<Color> {
        let theme = THEME.get().expect("custom theme not set");
        theme.get_color(module, color)
    }
}

impl DefaultColors for CustomTheme {
    fn default_bg() -> Color {
        let theme = THEME.get().expect("custom theme not set");
        (&theme.defaults.bg).into()
    }

    fn default_fg() -> Color {
        let theme = THEME.get().expect("custom theme not set");
        (&theme.defaults.fg).into()
    }
}

impl CompleteTheme for CustomTheme {}

macro_rules! color_from_json {
    ($function:ident, $module:ident, $property:ident, $default:ident) => {
        fn $function() -> Color {
            Self::get_color(stringify!($module), stringify!($property))
                .unwrap_or_else(Self::$default)
        }
    };
}

impl CargoScheme for CustomTheme {
    color_from_json!(cargo_fg, cargo, fg, default_fg);
    color_from_json!(cargo_bg, cargo, bg, default_bg);
}

impl CmdScheme for CustomTheme {
    color_from_json!(cmd_passed_fg, cmd, passed_fg, default_fg);
    color_from_json!(cmd_passed_bg, cmd, passed_bg, default_bg);

    color_from_json!(cmd_failed_bg, cmd, failed_fg, default_fg);
    color_from_json!(cmd_failed_fg, cmd, failed_bg, default_bg);
}

impl CwdScheme for CustomTheme {
    fn path_bg_colors() -> Vec<Color> {
        vec![dark_grey()]
    }
}

impl LastCmdDurationScheme for CustomTheme {
    color_from_json!(time_bg, time, bg, default_bg);
    color_from_json!(time_fg, time, fg, default_fg);
}

impl ExitCodeScheme for CustomTheme {
    const EXIT_CODE_BG: Color = Color(0);
    const EXIT_CODE_FG: Color = Color(0);
}

impl GitScheme for CustomTheme {
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

impl PythonEnvScheme for CustomTheme {
    color_from_json!(pyenv_fg, py, venv_fg, default_fg);
    color_from_json!(pyenv_bg, py, venv_bg, default_bg);

    color_from_json!(pyver_fg, py, ver_fg, default_fg);
    color_from_json!(pyver_bg, py, ver_bg, default_bg);
}

impl ReadOnlyScheme for CustomTheme {
    const READONLY_FG: Color = Color(0);
    const READONLY_BG: Color = Color(0);
}

impl SpacerScheme for CustomTheme {}

impl HostScheme for CustomTheme {
    const HOSTNAME_FG: Color = Color(0);
    const HOSTNAME_BG: Color = Color(0);
}

impl UserScheme for CustomTheme {
    const USERNAME_ROOT_BG: Color = Color(0);
    const USERNAME_BG: Color = Color(0);
    const USERNAME_FG: Color = Color(0);
}

impl TimeScheme for CustomTheme {
    const TIME_BG: Color = Color(0);
    const TIME_FG: Color = Color(0);
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::Value;

use crate::colors::Color;
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
    modules: HashMap<String, HashMap<String, Value>>,
}

impl CustomThemeImpl {
    fn get_property(&self, module: &str, property: &str) -> Option<&Value> {
        self.modules
            .get(module)
            .and_then(|module| module.get(property))
    }
}

impl CustomTheme {
    pub fn load(path: PathBuf) {
        match serde_json::from_reader(File::open(path).unwrap()) {
            Ok(theme) => {
                let _ = THEME.set(theme);
            }
            Err(e) => {
                eprintln!("Failed to read theme.json: {}", e);
                if let Some(source) = e.source() {
                    eprintln!("{}", source);
                }
            }
        }

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
        theme
            .get_property(module, color)
            .map(|value| {
                serde_json::from_value::<ColorsJson>(value.to_owned())
                    .expect("property was not a color")
            })
            .map(|col_json| (&col_json).into())
    }

    pub fn get_colors(module: &str, property: &str) -> Option<Vec<Color>> {
        let theme = THEME.get().expect("custom theme not set");
        let value = theme.get_property(module, property);

        value.map(|value| {
            value
                .as_array()
                .expect("property is not an array")
                .iter()
                .map(|val| {
                    serde_json::from_value::<ColorsJson>(val.to_owned())
                        .expect("could not read value as color")
                })
                .map(|color_json| (&color_json).into())
                .collect()
        })
    }

    pub fn get_str(module: &str, property: &str) -> Option<String> {
        let theme = THEME.get().expect("custom theme not set");
        theme
            .get_property(module, property)
            .and_then(|value| value.as_str())
            .map(|s| s.to_string())
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
    color_from_json!(path_fg, cwd, path_fg, default_fg);
    fn path_bg_colors() -> Vec<Color> {
        Self::get_colors("cwd", "bg_colors").unwrap_or(vec![Self::default_bg()])
    }
}

impl LastCmdDurationScheme for CustomTheme {
    color_from_json!(time_bg, time, bg, default_bg);
    color_from_json!(time_fg, time, fg, default_fg);
}

impl ExitCodeScheme for CustomTheme {
    color_from_json!(exit_code_bg, exit_code, bg, default_bg);
    color_from_json!(exit_code_fg, exit_code, fg, default_fg);
}

impl GitScheme for CustomTheme {
    color_from_json!(git_remote_bg, git, remote_bg, default_bg);
    color_from_json!(git_remote_fg, git, remote_fg, default_fg);
    color_from_json!(git_staged_bg, git, staged_bg, default_bg);
    color_from_json!(git_staged_fg, git, staged_fg, default_fg);
    color_from_json!(git_notstaged_bg, git, notstaged_bg, default_bg);
    color_from_json!(git_notstaged_fg, git, notstaged_fg, default_fg);
    color_from_json!(git_untracked_bg, git, untracked_bg, default_bg);
    color_from_json!(git_untracked_fg, git, untracked_fg, default_fg);
    color_from_json!(git_conflicted_bg, git, conflicted_bg, default_bg);
    color_from_json!(git_conflicted_fg, git, conflicted_fg, default_fg);
    color_from_json!(git_repo_clean_bg, git, clean_bg, default_bg);
    color_from_json!(git_repo_clean_fg, git, clean_fg, default_fg);
    color_from_json!(git_repo_dirty_bg, git, dirty_bg, default_bg);
    color_from_json!(git_repo_dirty_fg, git, dirty_fg, default_fg);
}

impl PythonEnvScheme for CustomTheme {
    color_from_json!(pyenv_fg, py, env_fg, default_fg);
    color_from_json!(pyenv_bg, py, env_bg, default_bg);

    color_from_json!(pyver_fg, py, version_fg, default_fg);
    color_from_json!(pyver_bg, py, version_bg, default_bg);
}

impl ReadOnlyScheme for CustomTheme {
    color_from_json!(readonly_fg, readonly, fg, default_fg);
    color_from_json!(readonly_bg, readonly, bg, default_bg);
}

impl SpacerScheme for CustomTheme {
    color_from_json!(color_fg, spacer, fg, default_fg);
    color_from_json!(color_bg, spacer, bg, default_bg);
}

impl HostScheme for CustomTheme {
    color_from_json!(hostname_bg, hostname, bg, default_bg);
    color_from_json!(hostname_fg, hostname, fg, default_fg);
}

impl UserScheme for CustomTheme {
    color_from_json!(username_root_bg, username, root_bg, default_bg);
    color_from_json!(username_bg, username, bg, default_bg);
    color_from_json!(username_fg, username, fg, default_fg);
}

impl TimeScheme for CustomTheme {
    color_from_json!(time_bg, time, bg, default_bg);
    color_from_json!(time_fg, time, fg, default_fg);
}

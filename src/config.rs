use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub trait TerminalRuntimeMetadata {
    fn total_columns(&self) -> usize;
    fn last_command_duration(&self) -> Option<Duration>;
    fn last_command_status(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub rows: Vec<CommandLine>,
}

// single line of a command terminal
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandLine {
    pub left: Vec<LineSegment>,
    pub right: Option<Vec<LineSegment>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineSegment {
    SmallSpacer,
    LargeSpacer,
    Separator(SeparatorStyle),
    Cwd {
        max_length: usize,
        wanted_seg_num: usize,
        #[serde(default)]
        resolve_symlinks: bool,
    },
    ReadOnly,
    Git,
    PythonEnv,
    Cargo,
    Host,
    // Time { format: Option<String> },
    User,
    Cmd,
    LastCmdDuration {
        #[serde(deserialize_with = "deserialize_duration")]
        min_run_time: Duration,
    },
    Padding(usize),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeparatorStyle {
    Chevron,
    Round,
}

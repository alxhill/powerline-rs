use std::time::Duration;

use serde::{Deserialize, Serialize};

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
    Nvm,
    Cargo,
    Host,
    Time {
        format: Option<String>,
    },
    User,
    Cmd,
    LastCmdDuration {
        min_run_time: u64, // milliseconds
    },
    Padding(usize),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeparatorStyle {
    Chevron,
    Round,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: "rainbow".into(),
            rows: vec![
                CommandLine {
                    left: vec![
                        LineSegment::SmallSpacer,
                        LineSegment::ReadOnly,
                        LineSegment::Cwd {
                            max_length: 60,
                            wanted_seg_num: 5,
                            resolve_symlinks: false,
                        },
                        LineSegment::SmallSpacer,
                        LineSegment::Git,
                    ],
                    right: Some(vec![
                        LineSegment::Separator(SeparatorStyle::Round),
                        LineSegment::Cargo,
                        LineSegment::PythonEnv,
                        LineSegment::Padding(0),
                    ]),
                },
                CommandLine {
                    left: vec![
                        LineSegment::LastCmdDuration { min_run_time: 5 },
                        LineSegment::Cmd,
                    ],
                    right: None,
                },
            ],
        }
    }
}

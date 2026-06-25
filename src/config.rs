use std::time::Duration;

use serde::{Deserialize, Serialize};

pub trait TerminalRuntimeMetadata {
    fn shell_name(&self) -> String;
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
    Pr {
        /// Append a coloured dot reflecting the PR's CI check status. On by
        /// default; set to `false` to show just the PR number.
        #[serde(default = "default_true")]
        status: bool,
    },
    PythonEnv,
    Nvm,
    Sdkman,
    Cargo,
    Host,
    Shell,
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

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeparatorStyle {
    Chevron,
    Round,
    AngleLine,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: "rainbow".into(),
            rows: vec![
                CommandLine {
                    left: vec![
                        LineSegment::Padding(2),
                        LineSegment::Separator(SeparatorStyle::Round),
                        LineSegment::ReadOnly,
                        LineSegment::Cwd {
                            max_length: 60,
                            wanted_seg_num: 5,
                            resolve_symlinks: false,
                        },
                        LineSegment::Padding(2),
                        LineSegment::Git,
                        LineSegment::Pr { status: true },
                    ],
                    right: Some(vec![]),
                },
                CommandLine {
                    left: vec![
                        LineSegment::Shell,
                        LineSegment::LastCmdDuration { min_run_time: 50 },
                        LineSegment::Cmd,
                        LineSegment::Padding(1),
                    ],
                    right: Some(vec![
                        LineSegment::Separator(SeparatorStyle::Round),
                        LineSegment::Nvm,
                        LineSegment::Sdkman,
                        LineSegment::PythonEnv,
                        LineSegment::Cargo,
                        LineSegment::Padding(0),
                    ]),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The binary writes `Config::default()` to disk with `to_string_pretty`
    /// and then reads it back with `from_reader`. This guards that round-trip:
    /// the serialized default config must always deserialize into an equivalent
    /// `Config`, so a fresh install never ends up with an unparsable config.
    #[test]
    fn default_config_round_trips() {
        let default = Config::default();

        let json = serde_json::to_string_pretty(&default)
            .expect("default config should serialize to JSON");

        let parsed: Config =
            serde_json::from_str(&json).expect("serialized default config should parse back");

        // Compare via the canonical JSON form to confirm the round-trip is lossless.
        let reserialized = serde_json::to_string_pretty(&parsed)
            .expect("reparsed config should serialize to JSON");
        assert_eq!(json, reserialized);
    }
}

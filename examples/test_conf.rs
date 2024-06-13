use std::fs::File;
use std::time::Duration;

use powerline::config::{Config, TerminalRuntimeMetadata};
use powerline::themes::{RainbowTheme, SimpleTheme};
use powerline::Powerline;

struct FakeRuntimeMetadata;

impl TerminalRuntimeMetadata for FakeRuntimeMetadata {
    fn total_columns(&self) -> usize {
        100
    }

    fn last_command_duration(&self) -> Option<Duration> {
        None
    }

    fn last_command_status(&self) -> &str {
        "0"
    }
}

fn main() {
    let conf: Config = serde_json::from_reader(File::open("example_config.json").unwrap()).unwrap();

    for prompt in conf.rows {
        let powerline = match conf.theme.to_lowercase().as_str() {
            "rainbow" => Powerline::from_conf::<RainbowTheme>(&prompt, FakeRuntimeMetadata),
            "simple" => Powerline::from_conf::<SimpleTheme>(&prompt, FakeRuntimeMetadata),
            _ => panic!("Unknown theme"),
        };

        println!("{}", powerline.render(100));
    }
}

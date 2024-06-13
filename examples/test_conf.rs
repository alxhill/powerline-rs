use powerline::{config, Powerline};
use powerline::config::LineSegment::{Cmd, Cwd, Git, LastCmdDuration, PythonEnv};
use powerline::config::{CommandLine, LineSegment, SeparatorStyle};
use std::fs::File;
use std::time::Duration;
use LineSegment::{Padding, Separator};
use powerline::themes::{RainbowTheme, SimpleTheme};

fn main() {
    let conf = config::Config {
        theme: "rainbow".into(),
        rows: vec![
            CommandLine {
                left: vec![
                    Cwd {
                        max_length: 50,
                        wanted_seg_num: 4,
                        resolve_symlinks: false,
                    },
                    // Padding(2),
                    // Separator(SeparatorStyle::Round),
                    Git,
                ],
                right: Some(vec![PythonEnv]),
            },
            CommandLine {
                left: vec![
                    LastCmdDuration {
                        min_run_time: Duration::from_millis(10),
                    },
                    Cmd,
                ],
                right: None,
            },
        ],
    };

    let conf_output = serde_json::to_string_pretty(&conf).unwrap();
    println!("{}", conf_output);

    // let conf_input: config::Config =
    //     serde_json::from_reader(File::open("example_config.json").unwrap()).unwrap();

    let cmd_line = &conf.rows[0];

    let powerline = match conf.theme.to_lowercase().as_str() {
        "rainbow" => {
            Powerline::from_conf::<RainbowTheme>(cmd_line)
        }
        "simple" => {
            Powerline::from_conf::<SimpleTheme>(cmd_line)
        }
        _ => panic!("Unknown theme"),
    };

    println!("powerline: {}", powerline.render(100));
}

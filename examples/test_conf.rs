use std::fs::File;

use powerline::config::Config;
use powerline::Powerline;
use powerline::themes::{RainbowTheme, SimpleTheme};

fn main() {
    let conf: Config = serde_json::from_reader(File::open("example_config.json").unwrap()).unwrap();

    for prompt in conf.rows {
        let powerline = match conf.theme.to_lowercase().as_str() {
            "rainbow" => Powerline::from_conf::<RainbowTheme>(&prompt),
            "simple" => Powerline::from_conf::<SimpleTheme>(&prompt),
            _ => panic!("Unknown theme"),
        };

        println!("{}", powerline.render(100));
    }
}

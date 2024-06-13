use powerline::modules::*;
use powerline::powerline::Separator;
use powerline::powerline::{PowerlineRightBuilder, PowerlineLeftBuilder};
use powerline::themes::RainbowTheme;
use std::env;
use std::time::Duration;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let (status, duration, columns): (&str, &str, &str) = match args.as_slice() {
        [_, status, duration, columns] => (status, duration, columns),
        [_, status, duration] => (status, duration, "0"),
        _ => ("0", "0", "0"),
    };

    let columns = str::parse::<usize>(columns).unwrap_or(0);
    let duration = str::parse::<u64>(duration)
        .map(Duration::from_millis)
        .unwrap_or(Duration::from_secs(0));

    let top_prompt = powerline::Powerline::builder()
        .change_separator(Separator::Chevron)
        .add_module(Spacer::<RainbowTheme>::small())
        .add_module(Cwd::<RainbowTheme>::new(60, 5, false))
        .add_module(ReadOnly::<RainbowTheme>::new())
        .add_module(Spacer::<RainbowTheme>::small())
        .add_module(Git::<RainbowTheme>::new())
        .start_right()
        .change_separator(Separator::Round)
        .add_module(PythonEnv::<RainbowTheme>::new())
        .add_padding(0)
        .render(columns);

    let mini_prompt = powerline::Powerline::builder()
        .add_module(LastCmdDuration::<RainbowTheme>::new(
            duration, Duration::from_millis(0),
        ))
        .add_module(Cmd::<RainbowTheme>::new(status.to_owned()))
        .render(columns);

    println!("{}", top_prompt);
    print!("{} ", mini_prompt);
}

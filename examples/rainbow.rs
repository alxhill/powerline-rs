use chrono::Duration;
use powerline::modules::*;
use powerline::powerline::Separator;
use powerline::themes::RainbowTheme;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let (status, duration, columns): (&str, &str, &str) = match args.as_slice() {
        [_, status, duration, columns] => (status, duration, columns),
        [_, status, duration] => (status, duration, "0"),
        _ => ("0", "0", "0"),
    };

    let columns = str::parse::<usize>(columns).unwrap_or(0);
    let duration = str::parse::<i64>(duration)
        .map(Duration::milliseconds)
        .unwrap_or(Duration::seconds(0));

    let top_prompt = powerline::Powerline::new()
        .set_separator(Separator::Chevron)
        .add_module(Spacer::<RainbowTheme>::small())
        .add_module(Cwd::<6, RainbowTheme>::new(45, 4, false))
        .add_module(ReadOnly::<RainbowTheme>::new())
        .add_module(Spacer::<RainbowTheme>::small())
        .add_module(Git::<RainbowTheme>::new())
        .to_right()
        .set_separator(Separator::Round)
        .add_module(PythonEnv::<RainbowTheme>::new())
        .add_padding(0, None)
        .render(columns);

    let mini_prompt = powerline::Powerline::new()
        .add_module(LastCmdDuration::<RainbowTheme>::new(
            duration,
            Duration::milliseconds(0),
        ))
        .add_module(Cmd::<RainbowTheme>::new(status.to_owned()))
        .render(columns);

    println!("{}", top_prompt);
    print!("{} ", mini_prompt);
}

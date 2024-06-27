use std::env;
use std::time::Duration;

use powerline_rs::modules::*;
use powerline_rs::powerline::Separator;
use powerline_rs::powerline::{PowerlineLeftBuilder, PowerlineRightBuilder, PowerlineShellBuilder};
use powerline_rs::terminal::Shell;
use powerline_rs::themes::RainbowTheme;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let (status, duration, columns): (&str, &str, &str) = match args.as_slice() {
        [_, status, duration, columns] => (status, duration, columns),
        [_, status, duration] => (status, duration, "0"),
        _ => ("0", "0", "0"),
    };

    let columns = str::parse::<usize>(columns).unwrap_or(0);
    let duration = str::parse::<u64>(duration).map(Duration::from_millis).ok();

    powerline_rs::Powerline::builder()
        .set_shell(Shell::Bare) // override this to whatever shell you use
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

    powerline_rs::Powerline::builder()
        .set_shell(Shell::Bare) // override this to whatever shell you use
        .add_module(LastCmdDuration::<RainbowTheme>::new(
            duration,
            Duration::from_millis(0),
        ))
        .add_module(Cmd::<RainbowTheme>::new(status))
        .render(columns);
}

extern crate powerline;

use powerline::modules::Time;
use powerline::modules::*;
use powerline::theme::SimpleTheme;
use std::env::args;

fn main() {
    let prompt = powerline::Powerline::new()
        .add_module(Time::<SimpleTheme>::with_time_format("%H:%M:%S"))
        .add_module(User::<SimpleTheme>::new())
        .add_module(Host::<SimpleTheme>::new())
        .add_module(Cwd::<SimpleTheme>::new(45, 4, false))
        .add_module(Git::<SimpleTheme>::new())
        .add_module(ReadOnly::<SimpleTheme>::new())
        .add_module(Cmd::<SimpleTheme>::new(args().nth(1).unwrap_or("0".into())))
        .render(0);

    println!("{} ", prompt);
}

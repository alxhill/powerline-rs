use powerline::modules::*;
use powerline::powerline::PowerlineRightBuilder;
use powerline::themes::SimpleTheme;

fn main() {
    let prompt = powerline::Powerline::builder()
        .add_module(Cwd::<SimpleTheme>::new(45, 4, false))
        .add_module(Git::<SimpleTheme>::new())
        .add_module(ReadOnly::<SimpleTheme>::new())
        .add_module(Cmd::<SimpleTheme>::new("0"))
        .render(0);

    println!("{} ", prompt);
}

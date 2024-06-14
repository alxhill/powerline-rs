use powerline_rs::modules::*;
use powerline_rs::powerline::{PowerlineRightBuilder, PowerlineShellBuilder};
use powerline_rs::terminal::Shell;
use powerline_rs::themes::SimpleTheme;

fn main() {
    let prompt = powerline_rs::Powerline::builder()
        .set_shell(Shell::Bare) // override this to whatever shell you use
        .add_module(Cwd::<SimpleTheme>::new(45, 4, false))
        .add_module(Git::<SimpleTheme>::new())
        .add_module(ReadOnly::<SimpleTheme>::new())
        .add_module(Cmd::<SimpleTheme>::new("0"))
        .render(0);

    println!("{} ", prompt);
}

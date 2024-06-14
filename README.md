# powerline-rs

_Forked from [cirho/powerline-rust](https://github.com/cirho/powerline-rust) and adjusted for personal taste_

![Shell with pyenv showing](with_pyenv.png)

powerline-rs is a pure-rust version of [powerline-SHELL](https://github.com/b-ryan/powerline-shell). It's heavily
inspired
by it, but focuses on minimalizing time of execution and supporting a limited subset of features.

## Advantages

- blazing fast (~15ms when reading from a config file, 9ms for a compiled binary)
- runs backends only when needed (huge time improvements when not in a git repo or python venv)
- optional caching git results in memory or file
- supports fully compiled prompts (see `examples/rainbow.rs`) or can read from a provided config file.
- new themes and modules can be added easily (currently only Rainbow and Simple are included)
- supports multiline prompts as well as showing info on the right hand side of the terminal.

## Installation

powerline-rs relies on [Nerd Font](https://www.nerdfonts.com/) unicode characters - configure your terminal to use a
Nerd Font, otherwise most segments will not render correctly. Meslo LG S is recommended and can be
downloaded in patched form [here](https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Meslo.zip).

iTerm2 users are recommended to enable the "Use builtin Powerline glyphs" option even when using a Nerd Font as this
seems to fix some character alignment issues.

![iTerm2 Profile configuration](iterm_config.png)

Clone the repo and install for your desired SHELL:

```bash
git clone https://github.com/alxhill/powerline-rs
cd powerline-rs
cargo install --path .
```

You can also install (or create your own) compiled command by adding `--example {name}` to cargo command and
modifying your SHELL config to call the custom binary.

Make sure cargo's bin directory is in your `$PATH`.

### bash

```bash
source <(powerline init bash)
```

### zsh

```zsh
source <(powerline init zsh)
```

### fish

```bash
powerline init fish | source
```

## Customization

Powerline will create a default config file at $HOME/.config/powerline-rs/config.json. You can edit it to make changes,
which will be reflected immediately.

### Config file

`config.rs` has the full definition of all valid types in the config directory, `example_config.json` shows a complete
configuration setup.

The only two themes at the moment are "rainbow" and "simple". New themes must be added in code at the moment, and the
simple theme is not recommended.

### Custom program

You can also create a separate rust program to fully customize the appearance. This allows creating a new theme too.

```rust
use powerline::{modules::*, theme::SimpleTheme};

fn main() {
    let mut prompt = powerline::Powerline::new();

    prompt.add_module(User::<SimpleTheme>::new());
    prompt.add_module(Host::<SimpleTheme>::new());
    prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false));
    prompt.add_module(Git::<SimpleTheme>::new());
    prompt.add_module(ReadOnly::<SimpleTheme>::new());
    prompt.add_module(Cmd::<SimpleTheme>::new());

    println!("{}", prompt);
}


```

### Cache untracked files

Git module can be slower on repos with big number of untracked files. Read about caching untracked
files  [here](https://git-scm.com/docs/git-update-index).

### Custom theme

```rust
use powerline::{modules::*, terminal::Color};

struct Theme;

impl CmdScheme for Theme {
    const CMD_FAILED_BG: Color = Color(161);
    const CMD_FAILED_FG: Color = Color(15);
    const CMD_PASSED_BG: Color = Color(236);
    const CMD_PASSED_FG: Color = Color(15);
}

fn main() {
    let mut prompt = powerline::Powerline::new();
    prompt.add_module(Cmd::<SimpleTheme>::new());

    ...
```

TODO:

- Change git icon/name based on branch vs commit vs merging
- Add java / gradle / jenv / sdkman support
- Better multiline prompts (e.g lines between)
- Improve spacing / centering support
- Calculate column width more accurately

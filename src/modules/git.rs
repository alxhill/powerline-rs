use std::cmp::Ordering;
use std::env;
use std::marker::PhantomData;
use std::path::PathBuf;

use super::Module;
use crate::{Color, Powerline, Style};

#[cfg(not(feature = "libgit"))]
mod process;

#[cfg(not(feature = "libgit"))]
use process as internal;

#[cfg(feature = "libgit")]
mod libgit;

#[cfg(feature = "libgit")]
use libgit as internal;
use crate::powerline::Separator;

pub struct Git<S> {
    scheme: PhantomData<S>,
}

pub trait GitScheme {
    const GIT_AHEAD_BG: Color;
    const GIT_AHEAD_FG: Color;
    const GIT_BEHIND_BG: Color;
    const GIT_BEHIND_FG: Color;
    const GIT_STAGED_BG: Color;
    const GIT_STAGED_FG: Color;
    const GIT_NOTSTAGED_BG: Color;
    const GIT_NOTSTAGED_FG: Color;
    const GIT_UNTRACKED_BG: Color;
    const GIT_UNTRACKED_FG: Color;
    const GIT_CONFLICTED_BG: Color;
    const GIT_CONFLICTED_FG: Color;
    const GIT_REPO_CLEAN_BG: Color;
    const GIT_REPO_CLEAN_FG: Color;
    const GIT_REPO_DIRTY_BG: Color;
    const GIT_REPO_DIRTY_FG: Color;
}

impl<S: GitScheme> Default for Git<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: GitScheme> Git<S> {
    pub fn new() -> Git<S> {
        Git { scheme: PhantomData }
    }
}

pub struct GitStats {
    pub untracked: u32,
    pub conflicted: u32,
    pub non_staged: u32,
    pub ahead: u32,
    pub behind: u32,
    pub staged: u32,
    pub remote: bool,
    pub branch_name: String,
}

impl GitStats {
    pub fn is_dirty(&self) -> bool {
        (self.untracked + self.conflicted + self.staged + self.non_staged) > 0
    }
}

fn find_git_dir() -> Option<PathBuf> {
    let mut git_dir = env::current_dir().unwrap();
    loop {
        git_dir.push(".git/");

        if git_dir.exists() {
            git_dir.pop();
            return Some(git_dir);
        }
        git_dir.pop();

        if !git_dir.pop() {
            return None;
        }
    }
}

const UP_ARROW: char = '\u{2B06}';
const DOWN_ARROW: char = '\u{2B07}';
const TICK: char = '\u{2714}';
const PENCIL: char = '\u{270E}';
const QUESTION_MARK: char = '\u{2753}';
const FANCY_STAR: char = '\u{273C}';

const GITHUB_LOGO: char = '\u{e65b}';
const GIT_ICON: char = '\u{e0a0}';


impl<S: GitScheme> Module for Git<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let git_dir = match find_git_dir() {
            Some(dir) => dir,
            _ => return,
        };

        let stats = internal::run_git(&git_dir);

        let (branch_fg, branch_bg) = if stats.is_dirty() {
            (S::GIT_REPO_DIRTY_FG, S::GIT_REPO_DIRTY_BG)
        } else {
            (S::GIT_REPO_CLEAN_FG, S::GIT_REPO_CLEAN_BG)
        };

        let icons = if stats.remote {
            format!("{} {}", GIT_ICON, GITHUB_LOGO)
        } else { format!("{GIT_ICON}") };

        powerline.add_segment(format!("{} {}", icons, stats.branch_name), Style::custom(branch_fg, branch_bg, Separator::RoundRight));

        let mut add_elem = |count: u32, symbol, fg, bg| match count.cmp(&1) {
            Ordering::Equal => powerline.add_short_segment(symbol, Style::custom(fg, bg, Separator::RoundRight)),
            Ordering::Greater => powerline.add_short_segment(format!(" {} {}", count, symbol), Style::custom(fg, bg, Separator::RoundRight)),
            Ordering::Less => (),
        };

        add_elem(stats.ahead, UP_ARROW, S::GIT_AHEAD_FG, S::GIT_AHEAD_BG);
        add_elem(stats.behind, DOWN_ARROW, S::GIT_BEHIND_FG, S::GIT_BEHIND_BG);
        add_elem(stats.staged, TICK, S::GIT_STAGED_FG, S::GIT_STAGED_BG);
        add_elem(stats.non_staged, PENCIL, S::GIT_NOTSTAGED_FG, S::GIT_NOTSTAGED_BG);
        add_elem(stats.untracked, QUESTION_MARK, S::GIT_UNTRACKED_FG, S::GIT_UNTRACKED_BG);
        add_elem(stats.conflicted, FANCY_STAR, S::GIT_CONFLICTED_FG, S::GIT_CONFLICTED_BG);
    }
}

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Write as _;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::colors::Color;
use crate::themes::DefaultColors;
use crate::{Powerline, Style};

use super::Module;

/// How long a cached lookup stays fresh. Prompts rendered within this window
/// reuse the cache and never touch the network.
const CACHE_TTL: Duration = Duration::from_secs(60);
/// Debounce window for the background refresher, so several prompts rendered in
/// quick succession don't each spawn their own `gh` process.
const REFRESH_DEBOUNCE: Duration = Duration::from_secs(20);

/// Branches that never have a PR of their own - skip all work for these.
const SKIP_BRANCHES: &[&str] = &["develop", "main", "master", "HEAD"];

pub struct Pr<S> {
    scheme: PhantomData<S>,
}

pub trait PrScheme: DefaultColors {
    fn pr_fg() -> Color {
        Self::default_fg()
    }
    fn pr_bg() -> Color {
        Self::default_bg()
    }
    fn pr_icon() -> &'static str {
        "\u{ea64}" // nf-cod-git_pull_request
    }
}

impl<S: PrScheme> Default for Pr<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: PrScheme> Pr<S> {
    pub fn new() -> Pr<S> {
        Pr {
            scheme: PhantomData,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PrInfo {
    number: u64,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct PrCache {
    branch: String,
    /// `None` means "looked up, but no PR exists for this branch" - cached so we
    /// don't re-query on every prompt.
    pr: Option<PrInfo>,
    fetched_at: u64,
}

impl<S: PrScheme> Module for Pr<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let Some((branch, repo_root)) = current_branch_and_root() else {
            return;
        };
        if SKIP_BRANCHES.contains(&branch.as_str()) {
            return;
        }

        // Without a cache directory we'd have to fetch synchronously, which
        // could block the prompt on a network request - so bail instead.
        let Some(cache_path) = cache_path_for(&repo_root, &branch) else {
            return;
        };

        let cache = read_cache(&cache_path).filter(|c| c.branch == branch);

        // Refresh in the background when the cache is missing or stale. This
        // never blocks rendering - the result is picked up by a later prompt.
        if cache.as_ref().is_none_or(|c| is_stale(c.fetched_at)) {
            spawn_refresh(&branch, &repo_root, &cache_path);
        }

        // Render whatever we have right now (possibly slightly stale).
        if let Some(PrCache { pr: Some(pr), .. }) = cache {
            let label = format!("{} #{}", S::pr_icon(), pr.number);
            powerline.add_hyperlink_segment(&label, &pr.url, Style::simple(S::pr_fg(), S::pr_bg()));
        }
    }
}

/// Resolves the current branch name and repository root in a single, fast,
/// network-free git invocation. Returns `None` outside a git repository.
fn current_branch_and_root() -> Option<(String, PathBuf)> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD", "--show-toplevel"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let mut lines = text.lines();
    let branch = lines.next()?.trim().to_string();
    let root = lines.next()?.trim();

    if branch.is_empty() || root.is_empty() {
        return None;
    }

    Some((branch, PathBuf::from(root)))
}

fn cache_path_for(repo_root: &Path, branch: &str) -> Option<PathBuf> {
    let base = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))?;

    let mut hasher = DefaultHasher::new();
    repo_root.hash(&mut hasher);
    branch.hash(&mut hasher);

    Some(
        base.join("powerline-rs")
            .join(format!("pr-{:016x}.json", hasher.finish())),
    )
}

fn read_cache(path: &Path) -> Option<PrCache> {
    serde_json::from_reader(File::open(path).ok()?).ok()
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn is_stale(fetched_at: u64) -> bool {
    now_secs().saturating_sub(fetched_at) >= CACHE_TTL.as_secs()
}

/// True if a refresh was kicked off recently enough that we should let it
/// finish rather than spawning another one.
fn refresh_in_flight(lock_path: &Path) -> bool {
    fs::metadata(lock_path)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|elapsed| elapsed < REFRESH_DEBOUNCE)
}

/// Spawns a detached process to refresh the cache. The child's stdio is
/// redirected to null so the shell's command substitution doesn't block
/// waiting on the inherited pipe.
fn spawn_refresh(branch: &str, repo_root: &Path, cache_path: &Path) {
    let lock_path = cache_path.with_extension("lock");
    if refresh_in_flight(&lock_path) {
        return;
    }

    if let Some(parent) = cache_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    // Touch the lock up front to debounce concurrent prompts.
    let _ = File::create(&lock_path);

    let Ok(exe) = env::current_exe() else {
        return;
    };

    let _ = Command::new(exe)
        .arg("refresh-pr")
        .args(["--branch", branch])
        .arg("--repo-dir")
        .arg(repo_root)
        .arg("--cache")
        .arg(cache_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

/// Performs the blocking `gh` lookup and writes the cache. Invoked by the
/// hidden `refresh-pr` subcommand from the detached process spawned above.
pub fn refresh_pr(branch: &str, repo_dir: &Path, cache_path: &Path) {
    let cache = PrCache {
        branch: branch.to_string(),
        pr: fetch_pr(branch, repo_dir),
        fetched_at: now_secs(),
    };

    write_cache(cache_path, &cache);
    let _ = fs::remove_file(cache_path.with_extension("lock"));
}

fn fetch_pr(branch: &str, repo_dir: &Path) -> Option<PrInfo> {
    let output = Command::new("gh")
        .current_dir(repo_dir)
        .args(["pr", "view", branch, "--json", "number,url"])
        .output()
        .ok()?;

    // A non-zero exit usually just means there's no PR for this branch.
    if !output.status.success() {
        return None;
    }

    serde_json::from_slice(&output.stdout).ok()
}

fn write_cache(path: &Path, cache: &PrCache) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Write to a temp file and rename so a concurrent reader never sees a
    // half-written cache.
    let tmp = path.with_extension("tmp");
    if let Ok(mut file) = File::create(&tmp) {
        if serde_json::to_writer(&mut file, cache).is_ok() && file.flush().is_ok() {
            let _ = fs::rename(&tmp, path);
        }
    }
}

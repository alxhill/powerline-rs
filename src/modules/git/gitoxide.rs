use std::path::Path;

use gix::status::index_worktree::Item as IndexWorktreeItem;
use gix::status::plumbing::index_as_worktree::EntryStatus;
use gix::status::Item;

use super::GitStats;

/// gitoxide (pure-Rust) git backend. Produces the same [`GitStats`] the libgit
/// and CLI backends do: a count of staged / non-staged / untracked / conflicted
/// paths plus the ahead/behind distance from the upstream tracking branch.
pub fn run_git(path: &Path) -> GitStats {
    let repo = gix::discover(path).unwrap();

    let (mut untracked, mut staged, mut non_staged, mut conflicted) = (0u32, 0, 0, 0);

    // Walk the repository status: HEAD-tree-vs-index entries are staged, while
    // index-vs-worktree entries are either modifications, conflicts, renames,
    // or untracked directory contents. Untracked listing follows the repo's
    // `status.showUntrackedFiles` config (collapsed directories by default),
    // matching the other backends.
    let status = repo
        .status(gix::progress::Discard)
        .unwrap()
        .into_iter(None)
        .unwrap();

    for item in status {
        let Ok(item) = item else { continue };
        match item {
            Item::TreeIndex(_) => staged += 1,
            Item::IndexWorktree(IndexWorktreeItem::Modification { status, .. }) => {
                if matches!(status, EntryStatus::Conflict { .. }) {
                    conflicted += 1;
                } else {
                    non_staged += 1;
                }
            }
            Item::IndexWorktree(IndexWorktreeItem::Rewrite { .. }) => non_staged += 1,
            Item::IndexWorktree(IndexWorktreeItem::DirectoryContents { entry, .. }) => {
                if entry.status == gix::dir::entry::Status::Untracked {
                    untracked += 1;
                }
            }
        }
    }

    let head_name = repo.head_name().ok().flatten();
    // `None` for an unborn branch (no commits yet) as well as a missing HEAD.
    let head_id = repo.head_id().ok();

    let branch_name = match (&head_name, &head_id) {
        // On a branch with at least one commit: show the short branch name.
        (Some(name), Some(_)) => name.shorten().to_string(),
        // Detached HEAD: show the short commit hash.
        (None, Some(id)) => id.shorten_or_id().to_string(),
        // Unborn branch / no HEAD: match the libgit & CLI "Big Bang" label.
        _ => String::from("Big Bang"),
    };

    let (mut remote, mut ahead, mut behind) = (false, 0, 0);

    if let (Some(name), Some(local)) = (head_name, head_id) {
        let local = local.detach();
        let upstream = repo
            .branch_remote_tracking_ref_name(name.as_ref(), gix::remote::Direction::Fetch)
            .and_then(Result::ok)
            .and_then(|tracking| repo.find_reference(tracking.as_bstr()).ok())
            .and_then(|mut reference| reference.peel_to_id().ok().map(|id| id.detach()));

        if let Some(upstream) = upstream {
            remote = true;
            ahead = count_commits(&repo, local, upstream);
            behind = count_commits(&repo, upstream, local);
        }
    }

    GitStats {
        untracked,
        staged,
        non_staged,
        ahead,
        behind,
        conflicted,
        remote,
        branch_name,
    }
}

/// Count commits reachable from `tip` but not from `excluded`, i.e. the
/// `excluded..tip` range — equivalent to `git rev-list --count excluded..tip`.
fn count_commits(repo: &gix::Repository, tip: gix::ObjectId, excluded: gix::ObjectId) -> u32 {
    match repo.rev_walk(Some(tip)).with_hidden(Some(excluded)).all() {
        Ok(walk) => walk.filter(Result::is_ok).count() as u32,
        Err(_) => 0,
    }
}

# PowerShell support — testing plan

This document records what has been verified for the `pwsh` integration and lays
out a test plan for the platforms and terminals that can't be exercised from the
development machine.

## How the integration works

`superline install pwsh` appends a loader to the current-user PowerShell profile
(resolved by asking PowerShell itself for `$PROFILE.CurrentUserCurrentHost`, so
it is correct on every platform). The loader runs `superline init pwsh`, which
emits a `prompt` function. On each prompt PowerShell calls that function, which:

- captures `$?` **then** `$LASTEXITCODE` (order matters — any statement, even an
  assignment, resets `$?`) and maps them to a status string (`0` on success,
  the native exit code on failure, `1` for cmdlet failures with no exit code);
- reads the terminal width from `$Host.UI.RawUI.WindowSize.Width` (falling back
  to `80` when it isn't available, e.g. when output is redirected);
- reads the last command's duration in milliseconds from `Get-History`;
- invokes `superline show … pwsh <duration>` and returns its output joined with
  newlines (using `-join`, not `Out-String`, which would wrap/pad to the host
  width);
- restores `$LASTEXITCODE` so the user's value is not clobbered.

Internally `pwsh` maps to the same "bare escape" rendering mode as `fish`:
PSReadLine parses ANSI/VT escape sequences itself, so no `\[ \]` (bash) or
`%{ %}` (zsh) non-printing markers are emitted. As with `bash`, there is no
native right-hand prompt; the final prompt row shows only its left side.

## Verified on this machine (macOS, automated + manual)

Platform: macOS (darwin 25.5), PowerShell 7.5.4 (`pwsh`).

Automated (`cargo test`, see `tests/shell_rendering.rs`):

- [x] `pwsh` renders bare ANSI escapes, never bash/zsh markers.
- [x] `pwsh` output is byte-for-byte identical to `fish`.
- [x] bash/zsh keep their own marker styles (regression guard).
- [x] End-to-end: the generated `prompt` function loads in a real `pwsh`,
      renders a prompt, threads a failing native command's exit code into a red
      status segment, and preserves `$LASTEXITCODE`. (Skips cleanly if `pwsh`
      isn't on PATH, so CI without PowerShell still passes.)

Manual:

- [x] `superline init pwsh` emits a valid, loadable profile snippet.
- [x] `superline install pwsh` resolves `$PROFILE`, creates the
      `~/.config/powershell` directory and profile file when missing, and
      appends the loader; the resulting profile defines `prompt` and sets
      `$env:SUPERLINE`.
- [x] Status: failing native command (`sh -c 'exit 7'`) → `7` in the red
      segment; success → normal segment; failing cmdlet → `1`.
- [x] Duration arithmetic from `Start/EndExecutionTime` produces correct ms.
- [x] `$LASTEXITCODE` is preserved across prompt rendering.

## Test matrix for other platforms

Run the **functional checklist** below in each configuration.

| # | OS | PowerShell | Terminal | Notes |
|---|-----|-----------|----------|-------|
| 1 | Linux | `pwsh` 7+ | any modern (VTE/xterm) | Expected to match macOS exactly. |
| 2 | Windows 10/11 | `pwsh` 7+ | Windows Terminal | Primary Windows target; full Unicode + OSC 8. |
| 3 | Windows 10/11 | `pwsh` 7+ | VS Code integrated terminal | Common dev setup. |
| 4 | Windows 10/11 | Windows PowerShell 5.1 | Windows Terminal | Ships with the OS; PSReadLine 2.0. |
| 5 | Windows 10/11 | Windows PowerShell 5.1 | legacy `conhost.exe` | Verify VT processing + glyph fallback. |
| 6 | macOS | `pwsh` 7+ | iTerm2 / Terminal.app | Sanity re-check on a second terminal. |

> ⚠️ **Windows requires a binary port first.** The crate currently depends on the
> Unix-only `users` crate and reads `$HOME` directly (see "Known limitations").
> Until that work lands, configurations 2–5 cannot be built/run on Windows even
> though the PowerShell integration itself is correct. On macOS/Linux the
> integration is complete today.

## Functional checklist (per configuration)

Setup:

1. `cargo install --path .` (or copy the release binary) and ensure it is on
   `PATH` (`Get-Command superline`).
2. `superline install pwsh`, then open a new PowerShell session.

Checks:

- [ ] **Loads cleanly** — new session shows the prompt with no errors or stray
      text (the one-time "creating default conf" notice is expected only on the
      very first prompt).
- [ ] **cwd** renders, with `~` substituted for the home directory.
- [ ] **git** segment appears inside a repo (branch + dirty/ahead/behind state).
- [ ] **PR** segment appears for a branch with an open PR (requires `gh`), and
      the OSC 8 hyperlink is clickable in terminals that support it
      (Windows Terminal yes; legacy conhost no — should degrade to plain text).
- [ ] **environment** segments (python venv / cargo / node / java) show when
      applicable.
- [ ] **exit status** — run a failing native command and confirm the red status
      segment shows the code:
      - Windows: `cmd /c exit 5`
      - Unix: `sh -c 'exit 5'`
      Then run a successful command and confirm it returns to normal.
- [ ] **cmdlet failure** — `Get-Item C:\does-not-exist` (or `/nope`) shows a
      failure status (`1`).
- [ ] **command duration** — `Start-Sleep -Seconds 2` shows a duration segment
      on the next prompt; a fast command shows none (respecting `min_run_time`).
- [ ] **terminal width** — with a multi-row prompt that uses a `right` section,
      resize the window and confirm right-aligned segments track the new width
      and don't wrap.
- [ ] **multiline editing** — type a long command, use ↑/↓ history recall and
      `Ctrl+R` search; the prompt must not smear or miscalculate the cursor
      column (this exercises PSReadLine's ANSI-aware width handling).
- [ ] **glyphs** — separators and icons render with a Nerd Font; with a
      non-Nerd font they should fall back to boxes/blanks but not corrupt layout.
- [ ] **`$LASTEXITCODE` preserved** — run a native command, then on the next
      line `$LASTEXITCODE` still reports that command's code.
- [ ] **redirect safety** — `(prompt) | Out-Null` and running under a
      non-interactive host don't throw (WindowSize fallback path).
- [ ] **performance** — prompt latency feels instant in a git repo (cache warm).

### Windows PowerShell 5.1 specifics

- [ ] PSReadLine ≥ 2.0 is present (`Get-Module PSReadLine`); older 5.1 boxes may
      need `Install-Module PSReadLine`.
- [ ] ANSI/VT processing is enabled (Windows 10 1511+ enables it; legacy hosts
      may need `[console]::OutputEncoding` / VT enabling). If escapes show as
      literal text, that's the host, not superline.
- [ ] `$PROFILE.CurrentUserCurrentHost` resolves under
      `Documents\WindowsPowerShell\` (5.1) vs `Documents\PowerShell\` (7+); the
      installer should write to the correct one because it asks PowerShell.
- [ ] Output encoding is UTF-8 so Nerd Font glyphs aren't mangled
      (`$OutputEncoding` / `chcp 65001` for legacy conhost).

## Known limitations & follow-ups

- **Windows binary port (blocker for configs 2–5).** To build and run on
  Windows the crate needs:
  - `users` crate usage replaced with a cross-platform alternative. It's used
    for root detection in `src/modules/cmd.rs` (`get_current_uid() == 0`) and
    for the current user name in `src/modules/user.rs`. On Windows, gate these
    behind `#[cfg(unix)]` and provide a Windows path (e.g. default to
    non-root / use `USERNAME`, or detect elevation via the Win32 API).
  - `$HOME` reads replaced with a portable home lookup (`USERPROFILE` on
    Windows, or the `dirs` crate). Affected: config path in
    `src/bin/superline.rs` (`get_or_create_conf_file`), `~` substitution in
    `src/modules/cwd.rs`, and the PR cache dir in `src/modules/pr.rs`
    (`XDG_CACHE_HOME`/`HOME` → `LOCALAPPDATA` on Windows).
  These are pre-existing Unix assumptions shared by all shells, not introduced
  by the PowerShell work; the `pwsh` profile-path detection already avoids
  `$HOME`.
- **No native right prompt**, matching bash. Right-aligned segments only render
  on non-final rows of a multi-row prompt.
- **OSC 8 hyperlinks** (PR segment) require a terminal that supports them;
  legacy `conhost.exe` shows the label as plain text (no breakage).

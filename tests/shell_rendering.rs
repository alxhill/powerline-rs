//! Per-shell rendering checks for the compiled binary.
//!
//! Each shell needs its escape sequences wrapped differently so the shell
//! computes the prompt width correctly:
//!   * bash wraps non-printing sequences in `\[ ... \]` readline markers
//!   * zsh wraps them in `%{ ... %}`
//!   * fish and PowerShell emit bare ANSI/VT escapes (their line editors parse
//!     the escapes themselves)
//!
//! These tests pin that behaviour - in particular that PowerShell renders the
//! same bare escapes as fish - without needing any shell installed. The final
//! test additionally drives a real `pwsh` end-to-end when one is on PATH, and
//! skips otherwise so CI without PowerShell still passes.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_superline");

/// A throwaway `$HOME` so each render exercises the "create default config" path
/// in isolation. The pid + label keep parallel test threads from colliding.
fn scratch_home(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("superline-render-{}-{label}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create scratch home");
    dir
}

/// Render the default prompt for `shell` against the given `$HOME`, returning
/// the raw stdout (escape sequences and all).
fn render_in(home: &PathBuf, shell: &str) -> String {
    let output = Command::new(BIN)
        .args(["show", shell, "-s", "0", "-c", "80"])
        // Home lookup keys off $HOME on Unix and %USERPROFILE% on Windows.
        .env("HOME", home)
        .env("USERPROFILE", home)
        .output()
        .expect("failed to run the superline binary");
    assert!(
        output.status.success(),
        "`show {shell}` exited with failure\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Render against a fresh, pre-warmed `$HOME` so the one-time "creating default
/// config" notice (which embeds the home path) never appears in the output.
fn render(shell: &str) -> String {
    let home = scratch_home(shell);
    // Warm the config first so the notice is written and gone before we capture.
    let _ = render_in(&home, shell);
    let out = render_in(&home, shell);
    let _ = fs::remove_dir_all(&home);
    out
}

/// The real ESC byte that bare-escape shells (fish, PowerShell) emit.
const ESC: char = '\x1b';

#[test]
fn powershell_uses_bare_ansi_like_fish() {
    let pwsh = render("pwsh");
    assert!(
        pwsh.contains(ESC),
        "PowerShell prompt should contain raw ANSI escapes",
    );
    assert!(
        !pwsh.contains("\\["),
        "PowerShell prompt must not use bash's \\[ \\] readline markers",
    );
    assert!(
        !pwsh.contains("%{"),
        "PowerShell prompt must not use zsh's %{{ }} markers",
    );

    // PowerShell and fish both map to the same bare-escape mode internally, so
    // their output is identical except for the one place the default config
    // prints the shell's own name (the `shell` segment). Normalise that token
    // out of each and the rest must match byte-for-byte. Render both against one
    // pre-warmed home so the comparison isn't thrown off by per-home paths in
    // any notice text.
    let home = scratch_home("pwsh-vs-fish");
    let _ = render_in(&home, "pwsh"); // warm the config once
    let pwsh_render = render_in(&home, "pwsh").replace("pwsh", "<shell>");
    let fish_render = render_in(&home, "fish").replace("fish", "<shell>");
    assert_eq!(
        pwsh_render, fish_render,
        "PowerShell and fish should render identical bare-escape output",
    );
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn each_shell_uses_its_own_escape_style() {
    let bash = render("bash");
    assert!(
        bash.contains("\\[") && bash.contains("\\]"),
        "bash prompt should wrap escapes in \\[ \\] markers",
    );
    assert!(
        !bash.contains(ESC),
        "bash prompt should escape ESC as \\e, not emit a raw ESC byte",
    );

    let zsh = render("zsh");
    assert!(
        zsh.contains("%{") && zsh.contains("%}"),
        "zsh prompt should wrap escapes in %{{ }} markers",
    );
    assert!(
        !zsh.contains("\\["),
        "zsh prompt should not use bash's \\[ markers",
    );
}

/// The pwsh init must force the console to decode superline's UTF-8 output as
/// UTF-8. Without this, PowerShell decodes a native command's stdout using the
/// legacy OEM code page on Windows and mangles Nerd Font glyphs into mojibake
/// (the U+E0B0 separator shows up as `εé░`). This pins the directive so it can't
/// silently drop out of the generated snippet.
#[test]
fn powershell_init_forces_utf8_output_encoding() {
    let output = Command::new(BIN)
        .args(["init", "pwsh"])
        .output()
        .expect("failed to run `superline init pwsh`");
    assert!(output.status.success(), "`init pwsh` exited with failure");
    let init = String::from_utf8_lossy(&output.stdout);
    assert!(
        init.contains("[Console]::OutputEncoding") && init.contains("[System.Text.Encoding]::UTF8"),
        "pwsh init must set [Console]::OutputEncoding to UTF-8 so Nerd Font \
         glyphs aren't mangled; got:\n{init}",
    );
}

/// Returns true when a working `pwsh` is on PATH.
fn have_pwsh() -> bool {
    Command::new("pwsh")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// End-to-end: source the generated init snippet inside a real PowerShell, call
/// the `prompt` function it defines, and confirm it renders a prompt without
/// error. Skipped (passes) when `pwsh` is not installed.
#[test]
fn powershell_prompt_function_renders_end_to_end() {
    if !have_pwsh() {
        eprintln!("skipping powershell_prompt_function_renders_end_to_end: pwsh not on PATH");
        return;
    }

    let home = scratch_home("e2e");
    let bin_dir = PathBuf::from(BIN)
        .parent()
        .expect("binary has a parent dir")
        .to_path_buf();

    // Pre-create the default config so the one-time "creating default conf"
    // notice doesn't land in the captured prompt output.
    let warm = Command::new(BIN)
        .args(["show", "pwsh", "-s", "0", "-c", "80"])
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .output()
        .expect("warm up config");
    assert!(warm.status.success());

    // A native command that exits non-zero, to prove the status is threaded
    // through. `sh` may be absent on Windows, so use `cmd` there.
    let fail_cmd = if cfg!(windows) {
        "cmd /c exit 7"
    } else {
        "& sh -c 'exit 7'"
    };

    // Load the init snippet, then invoke the prompt function it defines.
    let script = r#"
        $env:PATH = $env:SLBIN + [IO.Path]::PathSeparator + $env:PATH
        (& superline init pwsh) -join "`n" | Invoke-Expression
        __FAILCMD__
        $out = prompt
        if ($out -notmatch [char]27) { Write-Error 'no ANSI escapes in prompt'; exit 1 }
        if ($out -notmatch '48;5;160m') { Write-Error 'failing command did not render a red status segment'; exit 1 }
        if ($LASTEXITCODE -ne 7) { Write-Error "LASTEXITCODE not preserved: $LASTEXITCODE"; exit 1 }
        Write-Host 'OK'
    "#
    .replace("__FAILCMD__", fail_cmd);

    let output = Command::new("pwsh")
        .args(["-NoProfile", "-Command", &script])
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("SLBIN", &bin_dir)
        .output()
        .expect("failed to run pwsh");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let _ = fs::remove_dir_all(&home);

    assert!(
        output.status.success() && stdout.contains("OK"),
        "pwsh prompt end-to-end failed\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
}

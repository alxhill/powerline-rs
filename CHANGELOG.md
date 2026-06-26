# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- *(pr)* wrap the PR segment's OSC 8 hyperlink in each shell's non-printing
  markers (`\[ \]` for bash, `%{ %}` for zsh). The hyperlink escapes were always
  emitted raw, so bash readline counted them - and the entire PR URL - as
  visible columns, corrupting cursor tracking and smearing the powerline glyph
  colours on redraw. For bash, ESC is now spelled `\e` and the ST terminator's
  backslash as `\\` so it doesn't merge with the `\]` close marker. fish and
  PowerShell were unaffected. ([#28](https://github.com/alxhill/superline/pull/28))

## [0.5.4](https://github.com/alxhill/superline/compare/v0.5.3...v0.5.4) - 2026-06-23

### Added

- *(shell)* fix powershell utf8 rendering ([#23](https://github.com/alxhill/superline/pull/23))
- *(git)* add gitoxide backend ([#21](https://github.com/alxhill/superline/pull/21))

### Fixed

- filter out empty dirs from gitoxide status ([#24](https://github.com/alxhill/superline/pull/24))
- *(install)* target the PowerShell edition install was run from ([#22](https://github.com/alxhill/superline/pull/22))
- *(platform)* link advapi32 on Windows for the libgit2 backend ([#19](https://github.com/alxhill/superline/pull/19))

### Fixed

- *(shell)* set `[Console]::OutputEncoding` to UTF-8 in the PowerShell init so
  Nerd Font glyphs and powerline separators aren't mangled into mojibake (e.g.
  `εé░`) when PowerShell decodes superline's output using the legacy OEM code page.

## [0.5.3](https://github.com/alxhill/superline/compare/v0.5.2...v0.5.3) - 2026-06-16

### Added

- *(platform)* add Windows compatibility ([#18](https://github.com/alxhill/superline/pull/18))
- *(shell)* add PowerShell (pwsh) support ([#17](https://github.com/alxhill/superline/pull/17))

### Other

- document custom theme JSON format ([#15](https://github.com/alxhill/superline/pull/15))

## [0.5.2](https://github.com/alxhill/superline/compare/v0.5.1...v0.5.2) - 2026-06-15

### Other

- use absolute image URLs in README so they render on crates.io ([#13](https://github.com/alxhill/superline/pull/13))

## [0.5.1](https://github.com/alxhill/superline/compare/v0.5.0...v0.5.1) - 2026-06-15

### Added

- *(pr)* only show status indicator for in-progress PRs ([#11](https://github.com/alxhill/superline/pull/11))

### Other

- add release-plz and conventional-commit PR title check ([#7](https://github.com/alxhill/superline/pull/7))
- install from crates.io and add a crates.io badge ([#10](https://github.com/alxhill/superline/pull/10))

# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-06-26

Focused AppImage installs on reliable dock grouping and launcher icons.

### Added

- Read `StartupWMClass` from inside the AppImage during install when a bundled `.desktop` file is available
- Smart fallback for dock grouping when the AppImage does not expose a desktop entry
- Install preview step for dock icon grouping
- `npm run release` to bump versions, tag, push, and publish automatically
- `scripts/release.mjs` and release notes generation for GitHub releases

### Changed

- Icons are copied with their real extension (`icon.png`, `icon.svg`, etc.) instead of always forcing `.png`
- Launcher `Icon=` paths now point at the copied icon file directly
- README and contributor docs now describe the automated release flow

### Removed

- Extra `.deb` dependencies that were only needed for manual dock detection (`xdotool`, `x11-utils`, `wmctrl`)

## [0.1.1] - 2026-06-26

Refocused the app on AppImage install and management only.

### Added

- Search on the Installed apps tab
- GitHub Actions release workflow to build and publish the `.deb` on version tags

### Changed

- README rewritten around the AppImage install story
- Simpler install preview and installed apps UI

### Removed

- Fix dock icon tab for third-party apps such as Bottles
- Desktop integration scanning, window detection, and manual dock repair commands
- `fix-dock-dialog` and related backend window-class tooling

## [0.1.0] - 2026-06-26

First public release for Linux.

### Added

- Install AppImages into `~/Applications/` with a desktop launcher
- Copy icons, set menu category, and create `~/.local/share/applications/*.desktop` entries
- Installed apps tab to edit details and remove managed apps
- Install preview before writing files
- Light/dark theme support
- `.deb` packaging for Ubuntu-style desktops

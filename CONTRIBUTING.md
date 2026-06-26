# Contributing to AppImage to Applications

Thank you for your interest in contributing!

## Development setup

1. Install [Rust](https://rustup.rs/) and [Node.js](https://nodejs.org/) (LTS recommended).
2. Install Linux build dependencies (see README).
3. Clone the repository and install dependencies:

```bash
npm install
```

4. Run the app in development mode:

```bash
npm run tauri dev
```

## Branch naming

- `feat/` — new features
- `fix/` — bug fixes
- `chore/` — tooling, dependencies, config
- `docs/` — documentation only

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: add install preview panel`
- `fix: handle missing icon extension`
- `chore: update tauri dependencies`

Keep commits small and focused on one logical change.

## Pull requests

1. Fork the repository and create a feature branch from `main`.
2. Make your changes with clear commit messages.
3. Ensure the project builds: `npm run build` and `cargo check` in `src-tauri`.
4. Open a pull request with a description of what changed and why.

## Shipping a release

Only maintainers need this. Do not bump versions or create tags manually.

1. Write release notes under `## [Unreleased]` in [CHANGELOG.md](CHANGELOG.md).
2. Run:

```bash
npm run release
```

GitHub Actions builds the `.deb` and publishes the release using the matching `CHANGELOG.md` section. See the README **Ship a release** section for details.

## Code style

- **Rust:** system logic lives in `src-tauri/src/linux/` and `commands/`; keep Tauri command handlers thin.
- **React:** use shadcn components; keep client-side logic in leaf components.
- **Transparency:** any filesystem operation must be visible to the user (preview or activity log).

## Scope

Version 1 is Linux-only and AppImage-only. Please open an issue before starting work on out-of-scope features (Windows/macOS, package management, non-AppImage executables).

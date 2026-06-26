# AppImage to Applications

A small Linux desktop app I built because AppImages annoyed me in a very specific way.

---

## Why I built this

I downloaded Godot as an AppImage. It ran fine. But it did not feel installed.

It was not in my applications menu the way I expected. I had to figure out what a `.desktop` file is, where it goes, how to add an icon, and why the dock sometimes shows a second icon when the app is running. None of that is hard once you know Linux — but I did not want to learn all of that just to use an app I already downloaded.

So I set up Godot manually. Folder in `~/Applications`, launcher file, icon, the usual stuff. That worked.

I built this app so I do not have to do that manually again — and so other people do not have to either.

---

## What it does

**Install tab** — pick an AppImage, pick an icon, give it a name, and install it. The app:

- puts everything in `~/Applications/{AppName}/`
- creates a `.desktop` launcher in `~/.local/share/applications/`
- copies your icon into the app folder and points the launcher at it
- sets `StartupWMClass` so the dock groups with one icon (read from the AppImage when possible)
- refreshes your applications menu

**Installed tab** — see what you installed here, edit name/description/category, or remove an app completely.

---

## What it does not do

- It is not a package manager.
- It is not Flatpak or Snap.
- It only handles AppImages.

It does one job: make portable apps behave like normal desktop apps.

---

## Where files go

```
~/Applications/Godot/
    Godot
    icon.png    # or icon.svg, depending on what you picked

~/.local/share/applications/godot.desktop
```

Apps installed through this tool are marked with `X-AppImage-To-Applications=true` in the launcher file so the app can find and manage them.

Removing an app deletes its folder and its `.desktop` file.

---

## Requirements

- Linux (tested on Ubuntu-style desktops)

---

## Install

Download the latest `.deb` from [Releases](https://github.com/thabanidev/appimage-to-applications/releases).

```bash
sudo dpkg -i appimage-to-applications_*.deb
sudo apt-get install -f   # if anything is missing
```

Or build it yourself — see below.

---

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://rustup.rs/)

Linux build dependencies:

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Run locally

```bash
npm install
npm run tauri dev
```

### Ship a release

One command bumps the version, commits, tags, pushes, and triggers GitHub Actions to build and publish the `.deb`:

```bash
npm run release
```

That bumps the **patch** version (`0.1.1` → `0.1.2`). For bigger bumps:

```bash
npm run release:minor
npm run release:major
```

You do not need to edit version files or create tags by hand. The workflow runs when the tag is pushed.

To rebuild an existing release from GitHub: **Actions → Release → Run workflow** and enter the tag (for example `v0.1.2`).

### Build the `.deb` locally

```bash
npm run build:release
```

Output: `src-tauri/target/release/bundle/deb/`

---

## Tech

- [Tauri](https://tauri.app/) + Rust for system stuff
- React + TypeScript + Tailwind for the UI

---

## License

MIT — see [LICENSE](LICENSE).

If you want to contribute, see [CONTRIBUTING.md](CONTRIBUTING.md).

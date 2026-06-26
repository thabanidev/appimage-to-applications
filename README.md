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
    icon.png

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

### Build the `.deb`

```bash
npm run build:release
```

Output: `src-tauri/target/release/bundle/deb/`

### Publish a GitHub release

1. Bump the version in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` so they all match.
2. Commit and push to `main`.
3. Tag the release and push the tag:

```bash
git tag v0.1.1
git push origin main
git push origin v0.1.1
```

GitHub Actions builds the `.deb` and attaches it to the release automatically when the tag is pushed.

To build locally without publishing, run `npm run build:release` and install the `.deb` from the output folder above.

---

## Tech

- [Tauri](https://tauri.app/) + Rust for system stuff
- React + TypeScript + Tailwind for the UI

---

## License

MIT — see [LICENSE](LICENSE).

If you want to contribute, see [CONTRIBUTING.md](CONTRIBUTING.md).

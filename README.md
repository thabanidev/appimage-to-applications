# AppImage to Applications

## Project Overview

### What is this?

AppImage to Applications is a small open-source Linux desktop utility that solves one very specific problem.

It takes an AppImage and turns it into something that behaves like every other application installed on the system.

After using the tool, the application should:

* Appear in the Applications menu.
* Be searchable from the desktop.
* Have a proper application icon.
* Launch like a normal application.
* Be pinnable to the dock.
* Be manageable later if the user wants to change or remove it.

The goal is not to create another package manager.

The goal is to make portable applications feel like installed applications.

---

# The Problem

One of the nicest things about Linux is that you can download a single AppImage file and immediately run an application without installing anything.

That is also one of the biggest frustrations.

After downloading an AppImage, it doesn't automatically feel like an installed application.

Instead, the user has to learn things like:

* What a `.desktop` file is.
* Where `.desktop` files live.
* How desktop launchers work.
* How to make a file executable.
* Where to keep AppImages.
* How to find or download an icon.
* Why the application doesn't appear in the Applications menu.

None of these things are particularly difficult once you know Linux.

The problem is that you shouldn't need to know any of them just because you downloaded an application.

This project exists because I experienced exactly that frustration.

I downloaded Godot.

It worked.

But it didn't feel installed.

After learning about `.desktop` files, launcher registration, icons and application folders, I realised that the process is actually quite simple.

The problem isn't complexity.

The problem is that the process is manual.

---

# The Goal

The goal is to make this process graphical, simple and repeatable.

Instead of opening a terminal and manually creating launcher files, the user should simply provide:

* The AppImage
* The application name
* A description
* An icon

Then press one button.

Everything else should happen automatically.

---

# What the application actually does

Behind the scenes, the application performs the same steps an experienced Linux user would normally perform manually.

For example:

1. Create an application folder.
2. Copy the AppImage into that folder.
3. Rename the AppImage if required.
4. Make it executable.
5. Copy the chosen icon.
6. Create the desktop launcher.
7. Register the launcher with the desktop environment.
8. Refresh the desktop launcher database if necessary.

The user never has to think about these steps.

---

# Project Philosophy

This project follows a few simple principles.

## Solve one problem well.

This project is not trying to replace package managers.

It is not trying to compete with Flatpak.

It is not trying to become another software store.

Its job is very small.

Take an AppImage and make it behave like a normal desktop application.

Nothing more.

---

## Be transparent

The application should never hide what it is doing.

Every action should be understandable.

The user should always know:

* where files are copied
* what launcher is created
* where icons are stored
* how to undo everything

Nothing magical.

Nothing hidden.

---

## Follow Linux conventions

The application should respect existing Linux conventions instead of inventing new ones.

If Linux expects desktop launchers in a certain location, use that location.

If icons belong somewhere, follow the convention.

The application should fit naturally into the operating system.

---

## Keep everything reversible

Nothing should permanently modify the user's system.

If an application is removed, AppImage to Applications should be able to remove everything it created.

The system should be left clean.

---

## Make Linux friendlier

This project isn't about removing Linux concepts.

It's about removing unnecessary friction.

Someone should be able to enjoy portable applications without first learning how desktop entries work.

If they later become curious, great.

But they shouldn't be forced to become a Linux expert just to make an application appear in their launcher.

---

# Scope of Version 1

Version 1 focuses only on AppImages.

No support for shell scripts.

No support for Python scripts.

No support for custom executables.

Those ideas may come later, but they are intentionally out of scope for the first release.

A focused tool is usually a better tool.

---

# Technology

The application will be built using:

* Tauri
* React
* TypeScript
* Tailwind CSS
* Rust (only where native system access is required)

The project will be open source from the beginning.

---

# Development Approach

This project should be developed using modern software engineering practices.

That includes:

* Small, focused commits.
* Feature branches.
* Clear commit messages.
* Consistent project structure.
* Separation between UI logic and system logic.
* Clean, maintainable code over clever code.

Every feature should have a clear purpose.

If a feature does not solve a real user problem, it probably doesn't belong in the project.

---

# Long-term Vision

Eventually, this tool should become the easiest way to make an AppImage feel like a first-class desktop application.

A user should never need to know what a `.desktop` file is.

They should simply download an AppImage, open AppImage to Applications, choose an icon, click one button, and continue using their application like any other program installed on their computer.

---

# How it works

## Storage layout

Each installed application lives in one folder:

```
~/Applications/Godot/
    Godot.AppImage
    godot.png
```

Desktop launchers are created in the standard user location:

```
~/.local/share/applications/godot.desktop
```

There is no separate registry file. Installed applications are discovered by scanning desktop entries marked with `X-AppImage-To-Applications=true`.

## Remove an application

Removing an app deletes its `~/Applications/{name}/` folder and its `.desktop` launcher.

---

# Development

## Prerequisites

- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://rustup.rs/)
- Linux build dependencies:

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

## Run in development

```bash
npm install
npm run tauri dev
```

## Build frontend only

```bash
npm run build
```

## Build .deb package

```bash
npm run tauri build
```

The `.deb` file is written to `src-tauri/target/release/bundle/deb/`.

Installing the `.deb` automatically pulls in `desktop-file-utils`, `xdotool`, and `x11-utils` for launcher refresh and optional dock grouping fixes.

### Dock grouping

Most apps work out of the box because the installer sets `StartupWMClass` automatically.

If an app still opens as a separate dock icon, use **Fix dock grouping** on the Installed tab. That feature is optional — only needed when the dock misbehaves.

---

# License

MIT — see [LICENSE](LICENSE).

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

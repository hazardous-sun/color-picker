# Color picker

A color picker tool developed in Rust.

## Dependencies

### xdotool

This project relies on the "xdotool" library for tracking the mouse position. Make sure to install it before running 
your program.

#### Arch Linux

```bash
pacman -S xdotool
```

#### Debian-based

```bash
apt install libxdo-dev
```

#### Fedora

```bash
dnf install libX11-devel libxdo-devel
```

#### Gentoo

```bash
emerge -a xdotool
```

### Screen capture tool

This project requires a screen capture tool accepted by screenshot-rs library. Please note that this is automatically
detected by the library based on your desktop environment.

Here is a list of the libraries required to run the application depending on your desktop environment:

1. Gnome: gnome-screenshot
2. KDE: spectacle
3. Sway: slurp
4. Generic: scrot
5. MacOS: screencapture

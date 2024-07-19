# Quickshot
A simple xdg-desktop-portal based screenshot tool

Supports any desktop with a screenshot portal it does not matter if its x11 or wayland.

## Examples
Take screenshot of whole screen and save in ~/Pictures/Screenshots/
```
quickshot -o "$(xdg-user-dir PICTURES)/Screenshots/$(date +%Y%m%d-%Hh%Mm%Ss)_quickshot.png"
```

Take screenshot of part of screen and save in ~/Pictures/Screenshots/
```
quickshot -imo "$(xdg-user-dir PICTURES)/Screenshots/$(date +%Y%m%d-%Hh%Mm%Ss)_quickshot.png"
```

Take screenshot of part of screen and pipe into image viewer
```
quickshot -impe simp
```

## Manual
```
quickshot(1)

NAME
       quickshot - A simple xdg-desktop-portal based screenshot tool

SYNOPSIS
       quickshot [-m|--modal] [-i|--interactive] [-o ] [-p|--pipe] [-e|--exec] [--generate-man] [-w|--wait] [-q|--quiet] [-h|--help] [-V|--version]

DESCRIPTION
       A simple xdg-desktop-portal based screenshot tool

OPTIONS
       -m, --modal
              Whether the dialog should be modal

       -i, --interactive
              Customize area before taking a screenshot

       -o     Output filepath for screenshot

       -p, --pipe
              Send image data over stdout

       -e, --exec
              Spawn child and pipe output to child

       --generate-man
              Generates manual page for quickshot

       -w, --wait
              Wait for child to exit and return exit status of child

       -q, --quiet
              Don't print path to stdout

       -h, --help
              Print help

       -V, --version
              Print version

VERSION
       v1.0.0

AUTHORS
       Axel Kappel
```

## Installation
```
cargo install quickshot
```
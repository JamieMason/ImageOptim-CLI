# ImageOptim-CLI

> Automates [ImageOptim](https://imageoptim.com/mac), [ImageAlpha](https://pngmini.com/), and [JPEGmini for Mac](https://itunes.apple.com/us/app/jpegmini/id498944723) to make batch optimisation of images part of your automated build process.

## Installation

```
npm install --global imageoptim-cli
```

## Usage

```
Usage: imageoptim [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  Glob patterns of images or directories to optimise

Options:
  -a, --imagealpha            Enable ImageAlpha
  -j, --jpegmini              Enable JPEGmini
  -I, --no-imageoptim         Disable ImageOptim
      --quality <MIN-MAX>     ImageAlpha quality range from 0-100 [default: 65-80]
      --speed <N>             ImageAlpha speed from 1 (brute-force) to 10 (fastest) [default: 1]
      --number-of-colors <N>  ImageAlpha palette size from 2-256 [default: 256]
      --batch-size <N>        How many images to process at a time [default: 3000]
      --dry-run               List images which would be optimised, without optimising them
      --json                  Output newline-delimited JSON instead of human-readable text
      --verbose               Output debug logging
  -S, --no-stats              Do not display file size savings and quality loss information
  -C, --no-color              Output to the terminal without colors
  -h, --help                  Print help
  -V, --version               Print version
```

## Examples

```bash
# Run ImageOptim.app over every image in current directory
imageoptim

# Run ImageAlpha.app and ImageOptim.app over every PNG in current directory
imageoptim --imagealpha '**/*.png'

# Run JPEGmini.app and ImageOptim.app over every JPG in current directory
imageoptim --jpegmini '**/*.jpg' '**/*.jpeg'

# Run JPEGmini.app over every JPG in current directory
imageoptim --jpegmini --no-imageoptim '**/*.jpg' '**/*.jpeg'

# Run ImageOptim.app over every image in a specific directory
imageoptim '~/Desktop'
```

## Requirements

macOS only. Each app is optional and must be installed separately:

- [ImageOptim](https://imageoptim.com/mac) — free
- [ImageAlpha](https://pngmini.com/) — free
- [JPEGmini](https://itunes.apple.com/us/app/jpegmini/id498944723), [JPEGmini Lite](https://itunes.apple.com/us/app/jpegmini-lite/id525742250), or [JPEGmini Pro](https://itunes.apple.com/us/app/jpegmini-pro/id887163276) — paid

### ⚠️ JPEGmini and support for assistive devices

JPEGmini has no API or command line interface, so this tool automates its GUI. For that to work, the program running imageoptim-cli (such as your terminal) needs permission to control your computer under **System Settings → Privacy & Security → Accessibility**.

## Development

This is a Rust rewrite of the original TypeScript + AppleScript implementation, distributed on npm as platform-specific binaries. See the `justfile` for available commands:

```bash
just test        # run all tests
just lint        # cargo check, fmt, clippy
just build-local # build and install the npm packages for this machine
just test-local  # smoke test the locally built npm package
```

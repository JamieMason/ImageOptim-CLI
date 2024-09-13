# ImageOptim-CLI

> Automates [ImageOptim](http://imageoptim.com), [ImageAlpha](http://pngmini.com), and [JPEGmini for Mac](http://jpegmini.com/mac) to make batch optimisation of images part of your automated build process.

[![NPM version](http://img.shields.io/npm/v/imageoptim-cli.svg?style=flat-square)](https://www.npmjs.com/package/imageoptim-cli)
[![NPM downloads](http://img.shields.io/npm/dm/imageoptim-cli.svg?style=flat-square)](https://www.npmjs.com/package/imageoptim-cli)
[![Build Status](https://github.com/JamieMason/ImageOptim-CLI/actions/workflows/ci.yaml/badge.svg)](https://github.com/JamieMason/ImageOptim-CLI/actions/workflows/ci.yaml)
[![Maintainability](https://api.codeclimate.com/v1/badges/c7f41a90fa2c975cfd03/maintainability)](https://codeclimate.com/github/JamieMason/ImageOptim-CLI/maintainability)

## Table of Contents

-   [📣 Summary](#-summary)
-   [🌩 Installation](#-installation)
-   [🕹 Usage](#-usage)
-   [⚠️ JPEGmini and support for assistive devices](#️-jpegmini-and-support-for-assistive-devices)
-   [💡 Related Projects](#-related-projects)
-   [❓ FAQs](#-faqs)
-   [⚙️ Contributing](#️-contributing)
-   [🙋🏿‍♀️ Getting Help](#♀️-getting-help)
-   [👀 Other Projects](#-other-projects)
-   [🤓 Author](#-author)

## 📣 Summary

While other image optimization tools are available from the command line, ImageOptim-CLI exists because the [current benchmarks](http://jamiemason.github.io/ImageOptim-CLI/) suggest that ImageOptim, ImageAlpha and JPEGmini currently outperform those alternatives over lossless and lossy optimizations.

ImageOptim-CLI is written in TypeScript and AppleScript but is distributed as a self-contained executable binary, you don't need Node.js installed to use ImageOptim-CLI.

Check out this short [video demo of ImageOptim-CLI](https://www.youtube.com/watch?v=HGBounRIzSs) to see how it works.

## 🌩 Installation

### [npm](https://npmjs.org/)

    npm install -g imageoptim-cli

### [homebrew](https://brew.sh/)

    brew update
    brew install imageoptim-cli

### Manual

Otherwise, you can install manually by downloading the latest release then adding ImageOptim-CLI to your [\\\\$PATH](https://en.wikipedia.org/wiki/PATH_(variable)).

    # go to home directory
    cd ~
    # download the tarball (change 3.0.7 to latest version if available)
    curl --output imageoptim-cli.tgz https://registry.npmjs.org/imageoptim-cli/-/imageoptim-cli-3.0.7.tgz
    # extract the tarball
    tar -xvzf ./imageoptim-cli.tgz
    # delete the tarball
    rm imageoptim-cli.tgz
    # rename the directory extracted from the tarball
    mv ./package ./imageoptim-cli
    # make imageoptim command available in your terminal
    export PATH=$PATH:imageoptim-cli/dist

> Saving somewhere in your home directory such as `~/imageoptim-cli` is recommended, but not essential. Saving to `/Applications` is **not** recommended, do not do this.

## 🕹 Usage

    $ imageoptim --help

      Usage: imageoptim [options] [patterns...]

      Options:

        -V, --version           output the version number
        -a, --imagealpha        enable ImageAlpha
        -j, --jpegmini          enable JPEGmini
        -C, --no-color          output to the terminal without colors
        -I, --no-imageoptim     disable ImageOptim
        -Q, --no-quit           do not quit apps once finished
        -S, --no-stats          do not display file size savings and quality loss information
        --number-of-colors <n>  ImageAlpha palette size, defaults to 256
        --quality <min>-<max>   ImageAlpha quality range from 0-100, defaults to 65-80
        --speed <n>             ImageAlpha speed from 1 (brute-force) to 10 (fastest), defaults to 1
        -h, --help              output usage information

      Supported Apps:

        ImageAlpha: https://pngmini.com
        ImageOptim: https://imageoptim.com
        JPEGmini Lite: https://itunes.apple.com/us/app/jpegmini-lite/id525742250
        JPEGmini Pro: https://itunes.apple.com/us/app/jpegmini-pro/id887163276
        JPEGmini: https://itunes.apple.com/us/app/jpegmini/id498944723

      Examples:

        Run ImageOptim.app over every image in current directory
        imageoptim

        Run ImageAlpha.app and ImageOptim.app over every PNG in current directory
        imageoptim --imagealpha '**/*.png'

        Run JPEGmini.app and ImageOptim.app over every JPG in current directory
        imageoptim --jpegmini '**/*.jpg' '**/*.jpeg'

        Run ImageOptim.app over every image in a specific directory
        imageoptim '~/Desktop'

## ⚠️ JPEGmini and support for assistive devices

You may be presented with the following message the first time you run ImageOptim-CLI with the `--jpegmini` flag.

> To automate JPEGmini we need to add Terminal.app (or iTerm.app etc) to the 'support for assistive devices' whitelist.

The JPEGmini OS X Apps don't include a command line API, so a real user is simulated by entering synthetic clicks and keyboard commands instead. This requires your permission and is easily set up in System Preferences as shown by these guides.

-   [Enable access for assistive devices in OS X](http://mizage.com/help/accessibility.html)
-   [OS X Mavericks: Enable access for assistive devices and applications](http://support.apple.com/en-us/HT6026)

## 💡 Related Projects

### Grunt Plugin

The ImageOptim-CLI [Grunt](http://gruntjs.com) plugin is [grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim).

### Comparison of image optimization tools

ImageOptim-CLI features in this comparison of the [performance of image optimisation tools](http://jamiemason.github.io/ImageOptim-CLI/) alongside Kraken.io, CodeKit, grunt-contrib-imagemin, Smush.it, and TinyPNG.

### Article for Smashing Magazine

[How Optimized Are Your Images? Meet ImageOptim-CLI, a Batch Compression Tool](http://www.smashingmagazine.com/2013/12/17/imageoptim-cli-batch-compression-tool/)

### Alfred Workflow

The ImageOptim-CLI Workflow for Alfred app is [alfred-image-optim-workflow](https://github.com/ramiroaraujo/alfred-image-optim-workflow)

## ❓ FAQs

### General

#### Do ImageOptim, ImageAlpha, or JPEGmini come bundled with the ImageOptim-CLI installation?

You will need to install these applications separately.

#### Do I have to pay to use ImageOptim-CLI?

The CLI, ImageOptim and ImageAlpha are all free. JPEGmini is a paid-for product but you can use ImageOptim-CLI and choose not to run JPEGmini.

#### The WebP image format looks promising, can you get ImageOptim-CLI to convert images to it?

WebP looks great and may well overtake the formats handled by ImageOptim-CLI, but converting images to WebP is outside ImageOptim-CLI's chosen remit.

#### Can you get ImageOptim-CLI to skip images it has already processed, if they haven't changed?

JPEGmini does this today, but for ImageOptim and ImageAlpha I feel a feature like this belongs in those applications rather than this automator.

### ImageOptim

#### ImageOptim makes the fans on my Mac run at full power.

Optimising images is a pretty intensive process, so instead of optimising one image at a time (which would take forever) — ImageOptim optimises many images at the same time until all of them are done.

A side effect of this is that the fans come on at full power to keep your machine cool while it's maxed out.

### ImageAlpha

#### I don't think ImageAlpha is running, I can't see anything.

ImageOptim-CLI uses ImageAlpha's internal installation of [pngquant](http://pngquant.org) so it's normal that nothing is shown on screen.

It's also possible that if you look in the [Activity Monitor](http://support.apple.com/kb/HT5890) you will not see `pngquant` displayed but it _is_ being run. In my experience it's only when you run ImageOptim-CLI on a very large number of PNGs that you have enough time to spot it. ensure that Activity Monitor's **Update Frequency** is set to **Very Often (1 sec)**.

### JPEGmini

#### Can I use ImageOptim-CLI with JPEGmini Lite, the free version of JPEGmini?

Yes.

#### I upgraded from JPEGmini Lite to JPEGmini but ImageOptim-CLI still says JPEGmini is not installed.

Performing the in-app upgrade leaves the app named as jpegmini-lite, so ImageOptim-CLI can't determine whether it's the free or full version. It is better to instead buy [the full version of JPEGmini](https://itunes.apple.com/us/app/jpegmini/id498944723) separately.

#### ImageOptim-CLI says “To automate JPEGmini we need to enable GUI Scripting”, how do I do that?

See this tutorial on [how to manage Accessibility preferences and GUI Scripting](http://www.macosautomation.com/mavericks/guiscripting/index.html). In the case of OS X Mavericks, you will want to add the Applications JPEGmini and Terminal (or equivalent such as iTerm).

### Windows and Linux

#### Can I use ImageOptim-CLI on Windows or Linux?

ImageOptim-CLI is responsible for automating 3 OS X applications so is inherently bound to OS X for that reason.

#### Are there any plans for ImageOptim-CLI to support Windows or Linux?

It would first require ImageOptim, ImageAlpha, and JPEGmini to be available for those platforms.

#### I don't have OS X, can you recommend an alternative to ImageOptim-CLI?

[@addyosmani](https://github.com/addyosmani) wrote a really thorough article on [tools for image optimization](http://addyosmani.com/blog/image-optimization-tools/) which discusses a wide range of options in great detail.

## ⚙️ Contributing

Have an idea? Found a bug? Please see the [Contributing Guide](/CONTRIBUTING.md) for information on how to install the project and start writing code.

## 🙋🏿‍♀️ Getting Help

Get help with issues by creating a [Bug Report] or discuss ideas by opening a [Feature Request].

[bug report]: https://github.com/JamieMason/ImageOptim-CLI/issues/new?template=bug_report.md

[feature request]: https://github.com/JamieMason/ImageOptim-CLI/issues/new?template=feature_request.md

## 👀 Other Projects

If you find my Open Source projects useful, please share them ❤️

-   [**eslint-formatter-git-log**](https://github.com/JamieMason/eslint-formatter-git-log)<br>ESLint Formatter featuring Git Author, Date, and Hash
-   [**eslint-plugin-move-files**](https://github.com/JamieMason/eslint-plugin-move-files)<br>Move and rename files while keeping imports up to date
-   [**eslint-plugin-prefer-arrow-functions**](https://github.com/JamieMason/eslint-plugin-prefer-arrow-functions)<br>Convert functions to arrow functions
-   [**Jasmine-Matchers**](https://github.com/JamieMason/Jasmine-Matchers)<br>Write Beautiful Specs with Custom Matchers
-   [**karma-benchmark**](https://github.com/JamieMason/karma-benchmark)<br>Run Benchmark.js over multiple Browsers, with CI compatible output
-   [**self-help**](https://github.com/JamieMason/self-help#readme)<br>Interactive Q&A Guides for Web and the Command Line
-   [**syncpack**](https://github.com/JamieMason/syncpack#readme)<br>Manage multiple package.json files, such as in Lerna Monorepos and Yarn Workspaces

## 🤓 Author

<img src="https://www.gravatar.com/avatar/acdf106ce071806278438d8c354adec8?s=100" align="left">

I'm [Jamie Mason] from [Leeds] in England, I began Web Design and Development in 1999 and have been Contracting and offering Consultancy as Fold Left Ltd since 2012. Who I've worked with includes [Sky Sports], [Sky Bet], [Sky Poker], The [Premier League], [William Hill], [Shell], [Betfair], and Football Clubs including [Leeds United], [Spurs], [West Ham], [Arsenal], and more.

<div align="center">

[![Follow JamieMason on GitHub][github badge]][github]      [![Follow fold_left on Twitter][twitter badge]][twitter]

</div>

<!-- images -->

[github badge]: https://img.shields.io/github/followers/JamieMason.svg?style=social&label=Follow

[twitter badge]: https://img.shields.io/twitter/follow/fold_left.svg?style=social&label=Follow

<!-- links -->

[arsenal]: https://www.arsenal.com

[betfair]: https://www.betfair.com

[github]: https://github.com/JamieMason

[jamie mason]: https://www.linkedin.com/in/jamiemasonleeds

[leeds united]: https://www.leedsunited.com/

[leeds]: https://www.instagram.com/visitleeds

[premier league]: https://www.premierleague.com

[shell]: https://www.shell.com

[sky bet]: https://www.skybet.com

[sky poker]: https://www.skypoker.com

[sky sports]: https://www.skysports.com

[spurs]: https://www.tottenhamhotspur.com

[twitter]: https://twitter.com/fold_left

[west ham]: https://www.whufc.com

[william hill]: https://www.williamhill.com

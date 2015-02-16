# ImageOptim-CLI

> Automates [ImageOptim](http://imageoptim.com), [ImageAlpha](http://pngmini.com), and
> [JPEGmini for Mac](http://jpegmini.com/mac) to make batch optimisation of images part of your
> automated build process.

While other image optimization tools are available from the command line, ImageOptim-CLI exists
because the [current benchmarks](http://jamiemason.github.io/ImageOptim-CLI/) suggest that
ImageOptim, ImageAlpha and JPEGmini currently outperform those alternatives over lossless and
lossy optimizations.

Check out this short [video demo of ImageOptim-CLI](https://www.youtube.com/watch?v=HGBounRIzSs) to
see how it works.

Jamie Mason / [@fold_left](https://twitter.com/fold_left).




## Contents

+ [Installation](#installation)
  + [Manual Installation](#manual-installation)
  + [JPEGmini and support for assistive devices](#jpegmini-and-support-for-assistive-devices)
+ [Usage](#usage)
+ [Examples](#examples)
  + [Optimise a directory of images](#optimise-a-directory-of-images)
  + [Optimise a filtered set of images](#optimise-a-filtered-set-of-images)
  + [Passing additional options](#passing-additional-options)
  + [Adding to git pre-commit hook](#adding-to-git-pre-commit-hook)
+ [Related projects](#related-projects)
  + [Grunt plugin](#grunt-plugin)
  + [Comparison of image optimization tools](#comparison-of-image-optimization-tools)
  + [Article for Smashing Magazine](#article-for-smashing-magazine)
  + [Alfred workflow](#alfred-workflow)
+ [FAQs](#faqs)
  + [General](#general)
  + [ImageOptim](#imageoptim)
  + [ImageAlpha](#imagealpha)
  + [JPEGmini](#jpegmini)
  + [Windows and Linux](#windows-and-linux)




## Installation

ImageOptim-CLI is written in Shell and AppleScript, you don't _need_ Node.js to run it.
[npm](https://npmjs.org/) is suggested because it makes installation very convenient.

```shell
npm install -g imageoptim-cli
```

### Manual Installation

You can install manually by downloading the latest zip then adding ImageOptim-CLI to your
[$PATH](https://en.wikipedia.org/wiki/PATH_\(variable\)).

```shell
curl --output imageoptim-cli.zip https://codeload.github.com/JamieMason/ImageOptim-CLI/zip/1.11.6
unzip imageoptim-cli.zip
export PATH=$PATH:imageoptim-cli/bin
```

> Saving somewhere in your home directory such as `~/imageoptim-cli` is recommended, but not
essential. Saving to `/Applications` is **not** recommended, do not do this.

### JPEGmini and support for assistive devices

You may be presented with the following message the first time you run ImageOptim-CLI with the `--jpeg-mini` flag.

> To automate JPEGmini we need to add Terminal.app (or iTerm.app etc) to the 'support for assistive devices' whitelist.

The JPEGmini OS X Apps don't include a command line API, so a real user is
simulated by entering synthetic clicks and keyboard commands instead. This
requires your permission and is easily set up in System Preferences as shown by
these guides.

+ [Enable access for assistive devices in OS X Yosemite](http://www.klabouch.com/?p=98).
+ [OS X Mavericks: Enable access for assistive devices and applications](http://support.apple.com/en-us/HT6026)





## Usage
    Usage: imageoptim [options]
    
    Options:
    
      -d, --directory     directory of images to process
      -a, --image-alpha   pre-process PNGs with ImageAlpha.app *
      -j, --jpeg-mini     pre-process JPGs with JPEGmini.app **
      -q, --quit          quit all apps when complete
      -c, --no-color      disable color output
      -h, --help          display this usage information
      -e, --examples      display some example commands and uses
      -v, --version       display the version number
    
    *  http://pngmini.com
    ** https://itunes.apple.com/us/app/jpegmini/id498944723
    





## Examples
### Optimise a directory of images

This command will optimise all image files in your Awesome project.

```shell
imageoptim --directory ~/Sites/Awesome # [options]
```

### Optimise a filtered set of images

This command will optimise just the .jpg files in your Awesome project.

```shell
find ~/Sites/Awesome -name '*.jpg' | imageoptim # [options]
```

### Passing additional options

The long format for enabling options is as follows;

```shell
imageoptim --jpeg-mini --image-alpha --quit --no-color --directory path/to/images
```

The equivalent of the above in short format is as follows;

```shell
imageoptim -j -a -q -d -c path/to/images
```

### Adding to git pre-commit hook

Adding the below to **your_project/.git/hooks/pre-commit** will run imageoptim-CLI
each time you commit new and changed files into your project. Any files which
aren't images will be ignored.

```shell
images=$(git diff --exit-code --cached --name-only --diff-filter=ACM -- '*.png' '*.jpg')
$(exit $?) || echo $images | imageoptim && git add $images
```








## Related Projects


### Grunt Plugin

The ImageOptim-CLI [Grunt](http://gruntjs.com) plugin
is [grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim).


### Comparison of image optimization tools

ImageOptim-CLI features in this comparison of the [performance of image optimisation
tools](http://jamiemason.github.io/ImageOptim-CLI/) alongside Kraken.io, CodeKit,
grunt-contrib-imagemin, Smush.it, and TinyPNG.


### Article for Smashing Magazine

[How Optimized Are Your Images? Meet ImageOptim-CLI, a Batch Compression
Tool](http://www.smashingmagazine.com/2013/12/17/imageoptim-cli-batch-compression-tool/)


### Alfred Workflow

The ImageOptim-CLI Workflow for Alfred app
is [alfred-image-optim-workflow](https://github.com/ramiroaraujo/alfred-image-optim-workflow)




## FAQs





### General

#### Do ImageOptim, ImageAlpha, or JPEGmini come bundled with the ImageOptim-CLI installation?

You will need to install these applications separately.

#### Do I have to pay to use ImageOptim-CLI?

The CLI, ImageOptim and ImageAlpha are all free. JPEGmini is a paid-for product but you can use
ImageOptim-CLI and choose not to run JPEGmini.

#### Why are shorthand options supplied in the format `imageoptim -a -j -q` instead of the more common `imageoptim -ajq`?

This was an early mistake which now needs supporting for backwards compatibility.
[Issue 21](https://github.com/JamieMason/ImageOptim-CLI/issues/21) is open to add support for
the `-ajq` format as well as the current format.

#### The WebP image format looks promising, can you get ImageOptim-CLI to convert images to it?

WebP looks great and may well overtake the formats handled by ImageOptim-CLI, but converting images
to WebP is outside ImageOptim-CLI's chosen remit.

#### Can you get ImageOptim-CLI to skip images it has already processed, if they haven't changed?

JPEGmini does this today, but for ImageOptim and ImageAlpha I feel a feature like this belongs
in those applications rather than this automator.




### ImageOptim


#### ImageOptim makes the fans on my Mac run at full power.

Optimising images is a pretty intensive process, so instead of optimising one image at a time (which
would take forever) — ImageOptim optimises many images at the same time until all of them are done.

A side effect of this is that the fans come on at full power to keep your machine cool while it's
maxed out.





### ImageAlpha

#### I don't think ImageAlpha is running, I can't see anything.

ImageOptim-CLI uses ImageAlpha's internal installation of [pngquant](http://pngquant.org) so it's
normal that nothing is shown on screen.

It's also possible that if you look in the [Activity Monitor](http://support.apple.com/kb/HT5890)
you will not see `pngquant` displayed but it _is_ being run. In my experience it's only when you
run ImageOptim-CLI on a very large number of PNGs that you have enough time to spot it.

#### Can I configure ImageAlpha's settings using ImageOptim-CLI?

ImageOptim-CLI is intended to be a simple, convenient, powerful, general-purpose image optimisation
tool. If your needs have outgrown ImageOptim-CLI it's better to
run [pngquant](http://pngquant.org) separately, passing whatever custom settings you need.





### JPEGmini

#### Can I use ImageOptim-CLI with JPEGmini Lite, the free version of JPEGmini?

Not currently, the full version of JPEGmini already has some subtle differences and JPEGmini Lite
causes further fragmentation. It is possible but not a priority, as the free version has a limit of
processing 30 images per day anyway.

#### I upgraded from JPEGmini Lite to JPEGmini but ImageOptim-CLI still says JPEGmini is not installed.

Performing the in-app upgrade leaves the app named as jpegmini-lite, so ImageOptim-CLI can't
determine whether it's the free or full version. It is better to instead buy [the full version of
JPEGmini](https://itunes.apple.com/us/app/jpegmini/id498944723) separately.

#### ImageOptim-CLI says “To automate JPEGmini we need to enable GUI Scripting”, how do I do that?

See this tutorial on [how to manage Accessibility preferences
and GUI Scripting](http://www.macosautomation.com/mavericks/guiscripting/index.html). In the case
of OS X Mavericks, you will want to add the Applications JPEGmini and Terminal (or equivalent such
as iTerm).





### Windows and Linux

#### Can I use ImageOptim-CLI on Windows or Linux?

ImageOptim-CLI is responsible for automating 3 OS X applications so is inherently bound to OS X for
that reason.

#### Are there any plans for ImageOptim-CLI to support Windows or Linux?

It would first require ImageOptim, ImageAlpha, and JPEGmini to be available for those platforms.

#### I don't have OS X, can you recommend an alternative to ImageOptim-CLI?

[@addyosmani](https://github.com/addyosmani) wrote a really thorough article on [tools for
image optimization](http://addyosmani.com/blog/image-optimization-tools/) which discusses a wide
range of options in great detail.

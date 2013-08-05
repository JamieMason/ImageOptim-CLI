# ImageOptim-CLI

Automates [ImageOptim](http://imageoptim.com), [ImageAlpha](http://pngmini.com), and [JPEGmini for Mac](http://jpegmini.com/mac) to make batch optimisation of images part of your automated build process.

+ [Installation](#installation)
+ [Performance](#performance)
+ [Grunt Plugin](#grunt-plugin)
+ [Demo](#demo)
+ [Examples](#examples)
  + [Optimise a directory of images](#optimise-a-directory-of-images)
  + [Optimise a filtered set of images](#optimise-a-filtered-set-of-images)
  + [Passing additional options](#passing-additional-options)
  + [Adding to git pre-commit hook](#adding-to-git-pre-commit-hook)
+ [Usage](#usage)

## Installation

    $ npm install -g imageoptim-cli

ImageOptim-CLI is written in Shell and AppleScript, so you don't _need_ Node.js or npm. You can install manually by downloading the latest zip then adding ImageOptim-CLI to your [$PATH](https://en.wikipedia.org/wiki/PATH_\(variable\)).

	$ curl --output imageoptim-cli.zip https://codeload.github.com/JamieMason/ImageOptim-CLI/zip/{{version}}
	$ unzip imageoptim-cli.zip
	$ export PATH=$PATH:imageoptim-cli/bin

## Performance

See [how ImageOptim performs compared to other image optimisation tools](http://jamiemason.github.io/ImageOptim-CLI/).

[![](http://jamiemason.github.io/ImageOptim-CLI/static/image-optimisation-comparison.png)](http://jamiemason.github.io/ImageOptim-CLI/)

## Grunt Plugin

The ImageOptim-CLI [Grunt](http://gruntjs.com) plugin is [grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim).

## Demo

There's a short [video demo](https://www.youtube.com/watch?v=HGBounRIzSs) to give you an idea of how it works.

## Examples

{{examples}}

## Usage

{{usage}}

# ImageOptim-CLI

Automates batch image processing with [ImageOptim](http://imageoptim.com), [ImageAlpha](http://pngmini.com), and [JPEGmini for Mac](http://jpegmini.com/mac) to make lossless optimisation of images part of your automated build process.

## Installation

    $ npm install -g imageoptim-cli

ImageOptim-CLI is written in Shell and AppleScript, so you don't _need_ Node.js or npm. You can install manually by downloading the latest zip then adding ImageOptim-CLI to your [$PATH](https://en.wikipedia.org/wiki/PATH_\(variable\)).

	$ curl --output imageoptim-cli.zip https://codeload.github.com/JamieMason/ImageOptim-CLI/zip/{{version}}
	$ unzip imageoptim-cli.zip
	$ export PATH=$PATH:imageoptim-cli/bin

## Grunt Plugin

The ImageOptim-CLI [Grunt](http://gruntjs.com) plugin is [grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim).

## Demo

There's a short [video demo](https://www.youtube.com/watch?v=HGBounRIzSs) to give you an idea of how it works.

## Examples

{{examples}}

## Usage

{{usage}}

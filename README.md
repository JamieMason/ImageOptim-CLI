# ImageOptim-CLI

Automates batch image processing with [ImageOptim](http://imageoptim.com), [ImageAlpha](http://pngmini.com), and [JPEGmini for Mac](http://jpegmini.com/mac) to make lossless optimisation of images part of your automated build process.

## Example

    $ imageOptim --directory /path/to/images
	  Processing 57 images...
	  ✔ Finished in 54 seconds

You can see also a [video screen recording of ImageOptim-CLI](https://www.youtube.com/watch?v=HGBounRIzSs) on YouTube.

## Installation

    $ npm install -g imageoptim-cli

## Usage

    Usage: imageOptim [options]

    Options:

      -d, --directory     directory of images to process
      -a, --image-alpha   pre-process PNGs with ImageAlpha.app (http://pngmini.com)
      -j, --jpeg-mini     process JPGs with JPEGmini.app (https://itunes.apple.com/us/app/jpegmini/id498944723
      -q, --quit          quit ImageOptim.app when complete
      -h, --help          output usage information
      -e, --examples      output usage examples
      -v, --version       output the version number

## Examples

    Run ImageAlpha, ImageOptim & JPEGmini
    $ imageOptim -j -a -d path/to/images
    $ imageOptim --jpeg-mini --image-alpha --directory path/to/images

    Run ImageAlpha & ImageOptim
    $ imageOptim -a -d path/to/images
    $ imageOptim --image-alpha --directory path/to/images

    Run ImageOptim
    $ imageOptim -d path/to/images
    $ imageOptim --directory path/to/images

    Run ImageAlpha, ImageOptim, JPEGmini & quit them when finished
    $ imageOptim -j -q -a -d path/to/images
    $ imageOptim --jpeg-mini --quit --image-alpha --directory path/to/images

    Run ImageAlpha, ImageOptim & quit them when finished
    $ imageOptim -q -a -d path/to/images
    $ imageOptim --quit --image-alpha --directory path/to/images

    Run ImageOptim & quit it when finished
    $ imageOptim -q -d path/to/images
    $ imageOptim --quit --directory path/to/images

## Grunt Plugin

[JamieMason/grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim) is the [Grunt](http://gruntjs.com) plugin for imageoptim-cli.

## Credits

ImageOptim-CLI is the work of [Jamie Mason](https://github.com/JamieMason) and [James Stout](https://github.com/jamesstout). It extends [PorneL](https://github.com/pornel)'s brilliant [ImageOptim](https://github.com/pornel/ImageOptim) GUI Application for the Mac.

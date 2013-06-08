# ImageOptim-CLI

Controls [ImageOptim](http://imageoptim.com) and [ImageAlpha](http://pngmini.com) to make lossless optimisation of images part of your automated build process. Version 1.4 will soon also bring support for automating [JPEGmini for Mac](http://jpegmini.com/mac).

## Example

    $ imageOptim --directory /path/to/images
	  Processing 23 images...
	  ✔ Finished in 212 seconds

## Installation

    $ sudo npm install -g imageoptim-cli
    
## Usage

	  Usage: imageOptim [options]
	  
	  Options:
	
	    -d, --directory     directory of images to process
	    -a, --image-alpha   pre-process PNGs with ImageAlpha.app (http://pngmini.com)
	    -q, --quit          quit ImageOptim.app when complete
	    -h, --help          output usage information
	    -v, --version       output the version number

### Examples

#### Run ImageOptim

`imageOptim -d path/to/images` or `imageOptim --directory path/to/images`

#### Run ImageOptim then quit it when finished

`imageOptim -q -d path/to/images` or `imageOptim --quit --directory path/to/images`

#### Run ImageAlpha then ImageOptim

`imageOptim -a -d path/to/images` or `imageOptim --image-alpha --directory path/to/images`

#### Run ImageAlpha then ImageOptim then quit ImageOptim when finished

`imageOptim -q -a -d path/to/images` or `imageOptim --quit --image-alpha --directory path/to/images`

## Grunt Plugin

[JamieMason/grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim) is the [Grunt](http://gruntjs.com) plugin for imageoptim-cli.

## Credits

ImageOptim-CLI is the work of [Jamie Mason](https://github.com/JamieMason) and [James Stout](https://github.com/jamesstout). It extends [PorneL](https://github.com/pornel)'s brilliant [ImageOptim](https://github.com/pornel/ImageOptim) GUI Application for the Mac.

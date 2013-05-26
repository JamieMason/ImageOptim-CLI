# ImageOptim-CLI

Controls [ImageOptim](http://imageoptim.com) and [ImageAlpha](http://pngmini.com) to make lossless optimisation of images part of your automated build process.

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
	  -h, --help          output usage information
	  -v, --version       output the version number

## Grunt Plugin

[JamieMason/grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim) is the [Grunt](http://gruntjs.com) plugin for imageoptim-cli.

## Credits

ImageOptim-CLI is the work of [Jamie Mason](https://github.com/JamieMason) and [James Stout](https://github.com/jamesstout). It extends [PorneL](https://github.com/pornel)'s brilliant [ImageOptim](https://github.com/pornel/ImageOptim) GUI Application for the Mac.

## Similar Tools

Another web optimisation tool is [JamieMason/Unreadable](https://github.com/JamieMason/Unreadable), a CSS-aware HTML minifier and optimizer for the command line.

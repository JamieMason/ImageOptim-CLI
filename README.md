# ImageOptim-CLI

Automates batch image processing with [ImageOptim](http://imageoptim.com), [ImageAlpha](http://pngmini.com), and [JPEGmini for Mac](http://jpegmini.com/mac) to make lossless optimisation of images part of your automated build process.

## Installation

    $ npm install -g imageoptim-cli

## Grunt Plugin

If you use [Grunt](http://gruntjs.com) the ImageOptim-CLI plugin is [grunt-imageoptim](https://github.com/JamieMason/grunt-imageoptim).

## Demo

There's a [short video on YouTube](https://www.youtube.com/watch?v=HGBounRIzSs) to give you an idea of how it works.

## Usage

    Usage: imageOptim [options]
    
    Options:
    
      -d, --directory     directory of images to process
      -a, --image-alpha   pre-process PNGs with ImageAlpha.app *
      -j, --jpeg-mini     pre-process JPGs with JPEGmini.app **
      -q, --quit          quit all apps when complete
      -h, --help          display this usage information
      -e, --examples      display some example commands and uses
      -v, --version       display the version number
    
    *  http://pngmini.com
    ** https://itunes.apple.com/us/app/jpegmini/id498944723
    

## Examples

    Examples:
    
    OPTIMISE A DIRECTORY OF IMAGES
    ------------------------------
    This command will optimise all image files in your Awesome project.
    
        imageOptim --directory ~/Sites/Awesome # [options]
    
    OPTIMISE A FILTERED SET OF IMAGES
    ---------------------------------
    This command will optimise just the .jpg files in your Awesome project.
    
        find ~/Sites/Awesome -name '*.jpg' | imageOptim # [options]
    
    PASSING ADDITIONAL OPTIONS
    --------------------------
    The long format for enabling options is as follows;
    
        imageOptim --jpeg-mini --image-alpha --quit --directory path/to/images
    
    The equivalent of the above in short format is as follows;
    
        imageOptim -j -a -q -d path/to/images
    
    GIT PRE-COMMIT HOOK
    -------------------
    Adding the below to `your_project/.git/hooks/pre-commit` will run ImageOptim-CLI
    each time you commit new and changed files into your project. Any files which
    aren't images will be ignored.
    
        git diff --cached --name-only --diff-filter=ACM | imageOptim # [options]
    

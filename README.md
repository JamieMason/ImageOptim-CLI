ImageOptim-CLI
==============

**A Command Line version of [ImageOptim](http://imageoptim.com/) for the Mac.**

ImageOptim-CLI is a Shell Script which controls the same underlying executables as found in ImageOptim.app/Contents/MacOS/.

## Usage

    $ imageOptim /path/to/images

## Output

     FILE                                               KB REMOVED  % REMOVED
    -------------------------------------------------------------------------
     test/animated.gif                                3,689.165 KB     75.00%
     test/faustino-asprilla.jpg                           2.149 KB      1.00%
     test/tux.png                                         2.892 KB      6.00%
    -------------------------------------------------------------------------
     TOTAL                                            3,694.206 KB     73.00%

## Similar Projects

Similar projects exist such as **[toy/image\_optim](https://github.com/toy/image_optim)** so check that out as well.

The main differences in ImageOptim-CLI are;

1. it does not require you to install each of the optimisation tools yourself.
1. it does not depend on Ruby, it's just Shell.
1. it is new, and is still not yet stable.

Another web optimisation tool is **[JamieMason/Unreadable](https://github.com/JamieMason/Unreadable)**, a CSS-aware HTML minifier and optimizer for the command line.

## Please Contribute

I'm also a JavaScript Developer who is _fairly_ new to Shell, [forks & contributions](https://github.com/JamieMason/ImageOptim-CLI/pull/new/master) are welcomed.

## Known Issues

Paths containing spaces are not yet properly handled.

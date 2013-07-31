ImageOptim-CLI: gh-pages
===============

This is source for the project page for ImageOptim-CLI hosted at [http://jamiemason.github.io/ImageOptim-CLI](http://jamiemason.github.io/ImageOptim-CLI). 

## Sample images

A Photoshop .psd file was kindly given to us by [Daan Jobsis](http://www.twitter.com./daanjobsis) from his tests carried out for the article [Retina Revolution: Follow Up](http://blog.netvlies.nl/design-interactie/retina-revolutie-follow-up/). The sample set includes photographs of varying levels of detail, simple patterns, and logos.

The Photoshop layers were saved as pngs, gifs and jpgs in all possible combinations of progressive/not, with/without colour profiles interlaced/not etc.

## Re-running the test for yourself

1. Install this branch of ImageOptim-CLI

       # checkout project
       git clone -b gh-pages https://github.com/JamieMason/ImageOptim-CLI.git ImageOptim-Compare
       cd ImageOptim-Compare
       
       # clear out the original test images
       rm -rf images/codekit
       rm -rf images/grunt_contrib_imagemin
       rm -rf images/imageoptim_cli
       rm -rf images/imageoptim_cli_jpegmini
       rm -rf images/smushit
       rm -rf images/tinypng
       rm -rf images/photoshop
       rm -rf images/worst
       
       # we'll need these folders
       mkdir images/photoshop
       mkdir images/worst
       
       # install build tools
       cd src
       npm install
       cd ..

1. Fill "images/photoshop" and "images/worst" with the original images.

1. Create baseline worse possible quality images

       # PNGs
       find ./images/worst -type f -iname "*.png" -exec /Applications/ImageAlpha.app/Contents/Resources/pngquant 16 --quality=0-10 - < {} > {} \;
       
       # JPGs
       find ./images/worst -type f -iname "*.jpg" -exec convert -quality 10 {} {} \;
       
       # GIFs
       # @TODO: 31/Jul/2013

1. Duplicate and rename "images/photoshop" for every optimisation tool under test.

1. Optimise the contents of each folder using it's corresponding tool.

1. From the "src" directory, run `$ grunt data`, this will create [/src/raw-results.json](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/src/raw-results.json) and [/src/browser/results.js](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/src/browser/results.js).

1. From the "src" directory, run `$ grunt build` to regenerate [/index.html](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/index.html)

## Editing project page

### HTML

The HTML is edited in a Jade template at [/src/index.jade](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/src/index.jade), this makes it easy to output minified HTML and to inline the CSS and JS files into the document.

### JavaScript

#### App file

The app file to control the sortable, filterable table of test results is edited in [/src/browser/app.js](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/src/browser/app.js). AngularJS and the test results data are minfied and concatenated into [/src/browser/min.js](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/src/browser/min.js) then inlined into the Jade template using a Jade include.

### CSS

This is edited at [/src/browser/styles.css](https://github.com/JamieMason/ImageOptim-CLI/blob/gh-pages/src/browser/styles.css) then minified and inlined into the Jade template using a Jade include.

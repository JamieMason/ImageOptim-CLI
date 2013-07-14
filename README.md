ImageOptim-CLI: gh-pages
===============

This is source for the project page for ImageOptim-CLI hosted at [http://jamiemason.github.io/ImageOptim-CLI](http://jamiemason.github.io/ImageOptim-CLI). 

## Gathering Data

A Photoshop .psd file was kindly given to us by [Daan Jobsis](http://www.twitter.com./daanjobsis) from his tests carried out for the article [Retina Revolution: Follow Up](http://blog.netvlies.nl/design-interactie/retina-revolutie-follow-up/).

1. The Photoshop layers were saved as pngs, gifs and jpgs in all possible combinations of progressive or not, with/without colour profiles interlaced or not etc.

1. That folder of images was duplicated for every tool under test. 

1. Each folder was optimised using the respective tool.

1. Stats were gathered using `$ find images -type f -exec stat -f %z {} \;`

1. It's easily automated I know, but ultimately a manual step was involved to format that output into JSON.

1. That raw data at **/src/raw-results.json** was transformed into **/src/browser/results.js** to include % savings, which tool was the best etc using `$ node results/process-results.js`.

## Deploying project page

**/index.html** is generated using `grunt build`.

## Editing project page

### HTML

The HTML is edited in a Jade template at **/src/index.jade**, this makes it easy to output minified HTML and to inline the CSS and JS files into the document.

### JavaScript

#### App file

The app file to control the sortable, filterable table of test results is edited in **/src/browser/app.js**. AngularJS (**/src/browser/angular-1.0.7.min.js**) and the test results data are minfied and concatenated into **/src/browser/min.js** then inlined into the Jade template using a Jade include.

### CSS

This is edited at **/src/browser/styles.css** then minified and inlined into the Jade template using a Jade include.

### Cache Manifest

This is manually edited at **/app.appcache**, updating the timestamp invalidates the cache.
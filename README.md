ImageOptim-CLI: gh-pages
===============

This is source for the project page for ImageOptim-CLI hosted at [http://jamiemason.github.io/ImageOptim-CLI](http://jamiemason.github.io/ImageOptim-CLI). 

## Deploying

**/index.html** is generated using `grunt build`.

## Editing

### HTML

The HTML is edited in a Jade template at **/src/index.jade**, this makes it easy to output minified HTML and to inline the CSS and JS files into the document.

### JavaScript

#### App file

The app file to control the sortable, filterable table of test results is edited in **/src/browser/app.js**. AngularJS (**/src/browser/angular-1.0.7.min.js**) and the test results data are minfied and concatenated into **/src/browser/min.js** then inlined into the Jade template using a Jade include.

#### Benchmarks JSON data

The original results data is at **/src/raw-results.json**, this data is transformed into **/src/browser/results.js** to include % savings, which tool was the best etc. This transformation is done using `$ node results/process-results.js`

### CSS

This is edited at **/src/browser/styles.css** then minified and inlined into the Jade template using a Jade include.

### Cache Manifest

This is manually edited at **/app.appcache**, updating the timestamp invalidates the cache.
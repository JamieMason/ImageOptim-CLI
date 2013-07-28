module.exports = function(grunt) {

  'use strict';

  grunt.registerMultiTask('transform', 'Transform raw JSON results to a preferred form.', function() {

    var q = require('q');
    var _ = require('lodash');
    var toAbsolutePath = require('path').resolve;
    var raw = require(toAbsolutePath(this.data.src));
    var resultsFile = toAbsolutePath(this.data.dest);
    var writeFile = q.denodeify(require('fs').writeFile);

    q(raw)
      .then(function(list) {
        return _(list)

          // group tool results by image
          .groupBy(function(img) {
            return img.image;
          })

          // store tool results as array and lookup by tool
          .map(function(results, imageName, list) {
            return {
              all: results,
              index: _.reduce(results, function(memo, img) {
                memo[img.tool] = img;
                return memo;
              }, {})
            };
          })

          // work out which tool gained the highest file size decrease
          .each(function(img) {
            img.all = _(img.all)
              .sortBy(function(el) {
                return el.size;
              })
              .each(function(el, i, arr) {
                el.isSmallest = el.size < img.index.photoshop.size && (i === 0 || el.size === arr[0].size);
              })
              .value();
          })

          // store tools, image and size on a flat object
          .map(function(results) {
            var img = {};
            img.image = results.all[0].image;
            _.each(results.all, function(result) {
              img[result.tool] = result;
              delete result.image;
              delete result.tool;
            });
            img.size = img.photoshop.size;
            delete img.photoshop;
            return img;
          })

          // markup which tools don't handle certain image types
          .each(function(img) {
            var extension = img.image.split('.')[1];
            if (extension !== 'png') {
              img.tinypng.size = 'N/A';
              img.tinypng.meanErrorSquared = 'N/A';
              img.tinypng.sizeLoss = 'N/A';
              img.tinypng.sizeLossPercent = 'N/A';
              img.tinypng.qualityLossPercent = 'N/A';
              img.tinypng.isSmallest = 'N/A';
            }
            if (extension === 'gif') {
              img.smushit.size = 'N/A';
              img.smushit.meanErrorSquared = 'N/A';
              img.smushit.sizeLoss = 'N/A';
              img.smushit.sizeLossPercent = 'N/A';
              img.smushit.qualityLossPercent = 'N/A';
              img.smushit.isSmallest = 'N/A';
            }
          })

          // return value to promise
          .value();
      })

    // write JSON
    .then(function(collection) {
      return writeFile(resultsFile, 'var results = ' + JSON.stringify(collection, null, 2));
    })

    // finish
    .done(function() {
      console.log('SUCCESS');
    }, function() {
      console.log('FAIL');
    });

  });

};

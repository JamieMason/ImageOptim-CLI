module.exports = function(grunt) {

  'use strict';

  grunt.registerMultiTask('transform', 'Transform raw JSON results to a preferred form.', function() {

    var q = require('q');
    var _ = require('lodash');
    var toAbsolutePath = require('path').resolve;
    var raw = require(toAbsolutePath(this.data.src));
    var resultsFile = toAbsolutePath(this.data.dest);
    var writeFile = q.denodeify(require('fs').writeFile);
    var done = this.async();

    function markAsNotApplicable(result) {
      result.size = 'N/A';
      result.meanErrorSquared = 'N/A';
      result.sizeLoss = 'N/A';
      result.sizeLossPercent = 'N/A';
      result.qualityLossPercent = 'N/A';
      result.isSmallest = 'N/A';
    }

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
            .filter(function(el) {
              return el.tool !== 'worst';
            })
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
            markAsNotApplicable(img.tinypng);
          }
          if (extension === 'gif') {
            markAsNotApplicable(img.smushit);
            markAsNotApplicable(img.codekit);
            markAsNotApplicable(img.grunt_contrib_imagemin);
            markAsNotApplicable(img.kraken);
          }
        })

        // return value to promise
        .value();
      })

    // write JSON
    .then(function(collection) {
      return writeFile(resultsFile, 'var results = ' + JSON.stringify(collection, null, 2) + ';');
    })

    // finish
    .done(function() {
      console.log('SUCCESS');
      done(true);
    }, function() {
      console.log('FAIL');
      done(false);
    });

  });

};

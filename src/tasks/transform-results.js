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
      result.size = 0;
      result.meanErrorSquared = 0;
      result.sizeLoss = 0;
      result.sizeLossPercent = 0;
      result.qualityLossPercent = 0;
      result.isSmallest = false;
      result.exclude = true;
      result.classNames += ' na';
    }

    q(raw)
      .then(function(list) {
        return _(list)

        // group tool results by image
        .groupBy(function(img) {

          // rename keys
          if (img.tool === 'grunt_contrib_imagemin') {
            img.tool = 'grunt-contrib-imagemin';
          }
          if (img.tool === 'imageoptim_imagealpha') {
            img.tool = 'imagealpha-and-imageoptim';
          }
          if (img.tool === 'imageoptim_jpegmini') {
            img.tool = 'jpegmini-and-imageoptim';
          }

          return img.image;

        })

        // store tool results as array and lookup by tool
        .map(function(results, imageName, list) {
          return {
            all: results,
            index: _.reduce(results, function(memo, img) {
              img.exclude = false;
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
            .each(function(el) {
              el.score = el.sizeLossPercent - (el.qualityLossPercent * 2);
              el.isNoOp = el.sizeLoss === 0 && el.size > 0;
              el.isLossless = el.sizeLoss > 0 && el.qualityLossPercent === 0;
              el.isSmaller = el.size < img.index.photoshop.size;
              el.isDegraded = el.size > img.index.photoshop.size;
            })
            .sortBy(function(el) {
              return -el.score;
            })
            .each(function(el, i, arr) {
              el.hasBestScore = (i === 0 || el.score === arr[0].score);
              el.score = parseFloat(el.score.toFixed(2));
            })
            .sortBy(function(el) {
              return el.size;
            })
            .each(function(el, i, arr) {
              el.isSmallest = (i === 0 || el.score === arr[0].score);
            })
            .sortBy(function(el) {
              return -el.qualityLossPercent;
            })
            .each(function(el, i, arr) {
              var hasLoss = el.qualityLossPercent > 0;
              var hasSameLossAsFirst = el.qualityLossPercent === arr[0].qualityLossPercent;
              el.isLossiest = hasLoss && hasSameLossAsFirst;
            })
            .each(function(el) {
              el.classNames = [el.tool];
              if (el.hasBestScore) {
                el.classNames.push('best-score');
              }
              if (el.isSmallest) {
                el.classNames.push('smallest');
              }
              if (el.isNoOp) {
                el.classNames.push('noop');
              }
              if (el.isDegraded) {
                el.classNames.push('degraded');
              }
              if (el.isLossiest) {
                el.classNames.push('lossiest');
              }
              if (el.isLossless) {
                el.classNames.push('lossless');
              }
              el.classNames = el.classNames.join(' ');
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
            markAsNotApplicable(img['grunt-contrib-imagemin']);
            markAsNotApplicable(img.kraken);
          }
        })

        // return value to promise
        .value();
      })

    // write JSON
    .then(function(collection) {
      return writeFile(resultsFile, JSON.stringify(collection, null, 2));
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

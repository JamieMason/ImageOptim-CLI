module.exports = function(grunt) {

  'use strict';

  grunt.registerMultiTask('transform', 'Transform raw JSON results to a preferred form.', function() {

    var toAbsolutePath = require('path').resolve;
    var _ = require('lodash');
    var fs = require('fs');
    var raw = require(toAbsolutePath(this.data.src));
    var resultsFile = toAbsolutePath(this.data.dest);
    var allImages = [];
    var allGifs = [];
    var allJpgs = [];
    var allPngs = [];
    var indexAll = {};
    var json;
    var totals = {};
    var toolNames = [
      'codekit',
      'grunt_contrib_imagemin',
      'imageoptim_cli',
      'imageoptim_cli_jpegmini',
      'smushit',
      'tinypng'
    ];

    function getTotalTracker() {
      return {
        'size_codekit': 0,
        'size_grunt_contrib_imagemin': 0,
        'size_imageoptim_cli': 0,
        'size_imageoptim_cli_jpegmini': 0,
        'size_photoshop': 0,
        'size_smushit': 0,
        'size_tinypng': 0
      };
    }

    function hasExtension(extension, el) {
      return el.image.indexOf(extension) !== -1;
    }

    function addToTotal(totalKey, el) {
      totals[totalKey]['size_codekit'] += el['size_codekit'];
      totals[totalKey]['size_grunt_contrib_imagemin'] += el['size_grunt_contrib_imagemin'];
      totals[totalKey]['size_imageoptim_cli'] += el['size_imageoptim_cli'];
      totals[totalKey]['size_imageoptim_cli_jpegmini'] += el['size_imageoptim_cli_jpegmini'];
      totals[totalKey]['size_photoshop'] += el['size_photoshop'];
      totals[totalKey]['size_smushit'] += el['size_smushit'];
      totals[totalKey]['size_tinypng'] += el['size_tinypng'];
    }

    // get bytes removed and % bytes removed

    function getSavings(el, toolName) {
      el['diff_' + toolName] = el['size_photoshop'] - el['size_' + toolName];
      el['saving_' + toolName] = parseFloat(((el['diff_' + toolName] / el['size_photoshop']) * 100).toFixed(2));
    }

    function processResult(el) {
      // get number of bytes removed
      getSavings(el, 'codekit');
      getSavings(el, 'grunt_contrib_imagemin');
      getSavings(el, 'imageoptim_cli');
      getSavings(el, 'imageoptim_cli_jpegmini');
      getSavings(el, 'smushit');
      getSavings(el, 'tinypng');

      // order by biggest gains first
      el.best = _(toolNames).sortBy(function(toolName) {
        return el['diff_' + toolName];
      }).value().reverse();

      // keep best gaining tool or tools if any draw level
      el.best = _.filter(el.best, function(name) {
        var bestGain = el['diff_' + el.best[0]];
        var thisGain = el['diff_' + name];
        return bestGain !== 0 && thisGain === bestGain;
      });
    }

    // create Array of images with each tool's output
    _.each(raw, function(el, ix) {
      if (!indexAll[el.image]) {
        indexAll[el.image] = {
          image: el.image
        };
        allImages.push(indexAll[el.image]);
      }
      indexAll[el.image]['size_' + el.tool] = el.size;
    });

    // calculate savings
    _.each(allImages, processResult);

    // sort keys
    allImages = _.map(allImages, function(el, ix, list) {
      return _.reduce(Object.keys(el).sort(), function(memo, key) {
        memo[key] = el[key];
        return memo;
      }, {});
    });

    // add up totals
    totals.all = getTotalTracker();
    totals.jpg = getTotalTracker();
    totals.gif = getTotalTracker();
    totals.png = getTotalTracker();

    _(allImages).each(_.partial(addToTotal, 'all'));

    allGifs = _.filter(allImages, _.partial(hasExtension, '.gif'));
    allJpgs = _.filter(allImages, _.partial(hasExtension, '.jpg'));
    allPngs = _.filter(allImages, _.partial(hasExtension, '.png'));

    _.each(allGifs, _.partial(addToTotal, 'gif'));
    _.each(allJpgs, _.partial(addToTotal, 'jpg'));
    _.each(allPngs, _.partial(addToTotal, 'png'));

    // calculate total savings
    processResult(totals.all);
    processResult(totals.gif);
    processResult(totals.jpg);
    processResult(totals.png);

    // add N/A for images outside an app's scope
    function noTinyPng(el) {
      el.diff_tinypng = 'N/A';
      el.saving_tinypng = 'N/A';
      el.size_tinypng = 'N/A';
    }

    function noSmushit(el) {
      el.diff_smushit = 'N/A';
      el.saving_smushit = 'N/A';
      el.size_smushit = 'N/A';
    }

    // TinyPNG - png only
    _.each([].concat(allGifs, allJpgs), noTinyPng);
    noTinyPng(totals.jpg);
    noTinyPng(totals.gif);

    // Smushit, converts gifs to pngs
    _.each(allGifs, noSmushit);
    noSmushit(totals.gif);

    json = {
      total: totals,
      all: allImages
    };

    json = JSON.stringify(json, null, 2);

    json = 'var results = ' + json + ';';

    fs.writeFile(resultsFile, json, function(err) {
      if (err) {
        throw err;
      }
    });

  });

};

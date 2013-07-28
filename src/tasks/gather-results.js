module.exports = function(grunt) {

  'use strict';

  var _ = require('lodash');
  var q = require('q');
  var fs = require('fs');
  var toAbsolutePath = require('path').resolve;
  var exec = q.denodeify(require('child_process').exec);
  var writeFile = q.denodeify(require('fs').writeFile);

  grunt.registerTask('gather', 'Gather results from file system.', function() {

    var done = this.async();
    var imageDirectory = toAbsolutePath('../images');

    // get absolute paths to all files under /images
    exec('find ' + imageDirectory + ' -type f -exec echo {} \\;')

    .then(function(res) {

      return _(res[0].split('\n'))

      // remove eg .DS_Store files
      .filter(function(path) {
        return path && path.search(/\.(gif|jpg|jpeg|png)/) !== -1;
      })

      // convert image paths into objects
      .map(function(imgPath) {
        var branches = imgPath.split('/');
        return {
          path: imgPath,
          image: branches[branches.length - 1],
          tool: branches[branches.length - 2].replace(/\-/g, '_'),
          size: void(0),
          meanErrorSquared: void(0)
        };
      })

      .value();

    })

    // get the file sizes of each image
    .then(function(collection) {
      return collection.reduce(function(memo, img) {
        return memo.then(function() {
          return exec('stat -f %z ' + img.path).then(function(res) {
            img.size = parseInt(res[0], 10);
            return collection;
          });
        });
      }, q(null));
    })

    // compare the difference between the optimised image and the original
    .then(function(collection) {
      return collection.reduce(function(memo, img, i) {
        return memo.then(function() {
          var command = 'compare -metric MSE ' + imageDirectory + '/photoshop/' + img.image + ' ' + img.path + ' /dev/null';
          return exec(command)
            .then(function(res) {
              if (!res || !res[1]) {
                throw new Error('no output for ' + command);
              }
              img.meanErrorSquared = parseInt(res[1].replace(/ \(.+/, ''), 10);
              return collection;
            }, function(err) {
              var loss = err.toString().replace('Error: Command failed: ', '').replace(/ \(.+/, '');
              img.meanErrorSquared = parseFloat(loss, 10);
              return collection;
            });
        });
      }, q(null));
    })

    // calculate file size reduction
    .then(function (collection) {
      var originalSize = {};
      var results = [];

      // separe original from tools
      _.each(collection, function(img) {
        if (img.tool === 'photoshop') {
          originalSize[img.image] = img.size;
        } else {
          results.push(img);
        }
      });

      // calculate reduction
      _.each(results, function(img) {
        img.sizeLoss = originalSize[img.image] - img.size;
        img.sizeLossPercent = parseFloat(((img.sizeLoss / originalSize[img.image]) * 100).toFixed(2));
      });

      return collection;
    })

    // calculate % loss
    .then(function(collection) {
      var worstQuality = {};
      var results = [];

      // separe worst from tools
      _.each(collection, function(img) {
        if (img.tool === 'worst') {
          worstQuality[img.image] = img.meanErrorSquared;
        } else {
          results.push(img);
        }
      });

      // calculate loss
      _.each(results, function(img) {
        var imgLoss = img.meanErrorSquared;
        var worstLoss = worstQuality[img.image];
        var percentLoss = (imgLoss / worstLoss) * 100;

        // 0 / 0 will be NaN
        percentLoss = percentLoss || 0;

        img.qualityLossPercent = parseFloat(percentLoss.toFixed(2));
      });

      return collection;
    })

    // remove path property
    .then(function(collection) {
      return collection.map(function(img) {
        delete img.path;
        return img;
      });
    })

    // write to JSON
    .then(function(collection) {
      return writeFile('raw-results.json', JSON.stringify(collection, null, 2));
    })

    .done(function() {
      console.log('SUCCESS');
      done();
    }, function(err) {
      console.error('FAILED:', err);
      done();
    });

  });

};

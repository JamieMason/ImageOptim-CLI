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
    var images = toAbsolutePath('../images');
    var imageNames = [];
    var tools = [
      'codekit',
      'grunt-contrib-imagemin',
      'imageoptim-cli',
      'imageoptim-cli-jpegmini',
      'smushit',
      'tinypng'
    ];

    var failed = [];

    exec('find ' + images + ' -type f -exec echo {} \\;')
      .then(function(res) {
        return _(res[0].split('\n'))
          .filter(function(path) {
            return path && path.search(/\.(gif|jpg|jpeg|png)/) !== -1;
          })
          .map(function(imgPath) {
            var branches = imgPath.split('/');
            return {
              path: imgPath,
              image: branches[branches.length - 1],
              tool: branches[branches.length - 2],
              size: void(0),
              qualityLoss: void(0)
            };
          })
          .value();
      })
      .then(function(data) {
        return data.reduce(function(memo, img) {
          return memo.then(function() {
            return exec('stat -f %z ' + img.path).then(function(res) {
              var size = res[0];
              img.size = parseInt(size, 10);
              return data;
            });
          });
        }, q(null));
      })
      .then(function(data) {
        return data.reduce(function(memo, img, i) {
          return memo.then(function() {
            var command = 'compare -metric MSE ' + images + '/photoshop/' + img.image + ' ' + img.path + ' /dev/null';
            return exec(command)
              .then(function(res) {
                if (!res || !res[1]) {
                  throw new Error('no output for ' + command);
                }
                img.qualityLoss = parseInt(res[1].replace(/ \(.+/, ''), 10);
                return data;
              }, function(err) {
                var loss = err.toString().replace('Error: Command failed: ', '').replace(/ \(.+/, '');
                img.qualityLoss = parseFloat(loss, 10);
                return data;
              });
          });
        }, q(null));
      })
      .then(function (data) {
        return data.map(function (img) {
          delete img.path;
          return img;
        });
      })
      .then(function (data) {
        return writeFile('raw-results.json', JSON.stringify(data, null, 2));
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

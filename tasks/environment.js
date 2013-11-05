module.exports = function(grunt) {

  'use strict';

  var fs = require('fs');
  var path = require('path');

  function readFile(filePath, done) {
    fs.readFile(path.resolve(filePath), 'utf8', function(err, contents) {
      if (err) {
        throw err;
      }
      done(contents);
    });
  }

  function writeFile(filePath, contents, done) {
    fs.writeFile(path.resolve(filePath), contents, function(err) {
      if (err) {
        throw err;
      }
      done();
    });
  }

  function mergeObjectWithFile(filePath, data, onComplete) {
    readFile(filePath, function(contents) {
      Object.keys(data).forEach(function(token) {
        contents = contents.replace(new RegExp('\\{\\{' + token + '\\}\\}', 'g'), data[token]);
      });
      writeFile(filePath, contents, onComplete);
    });
  }

  function lineToShellEcho(line) {
    return '  echo "' + line + '"';
  }

  function lineToMarkdownCodeBlock(line) {
    return '    ' + line;
  }

  grunt.registerTask('environment', 'Apply environment config to build', function() {

    var data = {};
    var done = this.async();
    var paths = {
      examples: 'src/examples.md',
      imageOptim: 'bin/imageOptim',
      imageOptimBashLib: 'bin/imageOptimBashLib',
      imageOptimAppleScriptLib: 'bin/imageOptimAppleScriptLib',
      npmMeta: 'package.json',
      readme: 'README.md',
      usage: 'src/usage.txt'
    };

    Object.keys(paths).forEach(function(key) {
      paths[key] = path.join(process.cwd(), paths[key]);
    });

    data.version = require(paths.npmMeta).version;

    // get usage template
    readFile(paths.usage, function(usage) {

      // format usage appropriately for the terminal
      data.usage = usage.split('\n').map(lineToShellEcho).join('\n');

      // get usage examples
      readFile(paths.examples, function(examples) {

        // format usage examples appropriately for the terminal
        data.examples = examples.split('\n').map(lineToShellEcho).join('\n');

        // interpolate data with the imageoptim-cli bash script
        mergeObjectWithFile(paths.imageOptim, data, function() {

          // interpolate data with the imageoptim-cli bash library
          mergeObjectWithFile(paths.imageOptimBashLib, data, function() {

            // interpolate data with the imageoptim-cli applescript library
            mergeObjectWithFile(paths.imageOptimAppleScriptLib, data, function() {

              // format usage appropriately for the readme
              data.usage = usage.split('\n').map(lineToMarkdownCodeBlock).join('\n');

              // take original example usage data which is appropriate for the readme
              data.examples = examples;

              // interpolate data with the readme
              mergeObjectWithFile(paths.readme, data, function() {
                done();
              });

            });

          });

        });

      });

    });

  });

};

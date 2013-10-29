module.exports = function(grunt) {

  'use strict';

  grunt.initConfig({

    watch: {
      scripts: {
        files: ['src/**/*'],
        tasks: ['build'],
        options: {
          nospawn: true
        }
      }
    },

    concat: {
      dist: {
        files: [{
          src: 'src/imageOptimAppleScriptLib',
          dest: 'bin/imageOptimAppleScriptLib'
        }, {
          src: 'src/imageOptim',
          dest: 'bin/imageOptim'
        }, {
          src: 'src/README.md',
          dest: 'README.md'
        }]
      }
    }

  });

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

    var done = this.async();
    var data = {
      version: require('./package.json').version
    };

    readFile('src/usage.txt', function(usage) {

      data.usage = usage.split('\n').map(lineToShellEcho).join('\n');

      readFile('src/examples.md', function(examples) {

        data.examples = examples.split('\n').map(lineToShellEcho).join('\n');

        mergeObjectWithFile('bin/imageOptim', data, function() {
          mergeObjectWithFile('bin/imageOptimAppleScriptLib', data, function() {

            data.usage = usage.split('\n').map(lineToMarkdownCodeBlock).join('\n');
            data.examples = examples;

            mergeObjectWithFile('README.md', data, function() {
              done();
            });

          });
        });
      });
    });

  });

  grunt.loadNpmTasks('grunt-contrib-concat');
  grunt.loadNpmTasks('grunt-contrib-watch');

  grunt.registerTask('build', ['concat', 'environment']);

};

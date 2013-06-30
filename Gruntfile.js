module.exports = function(grunt) {

  'use strict';

  grunt.initConfig({

    environment: {
      base: require('./environment/base.json')
    },

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
          src: [
            'src/applescript/file_header.txt',
            'src/applescript/copyright.txt',
            'src/applescript/has_app_installed.applescript',
            'src/applescript/has_gui_script.applescript',
            'src/applescript/run_jpegmini.applescript',
            'src/applescript/wait_for.applescript',
            'src/applescript/handle_terminal_input.applescript'
          ],
          dest: 'bin/imageOptimAppleScriptLib'
        }, {
          src: [
            'src/shell/file_header.txt',
            'src/shell/copyright.txt',
            'src/shell/variables.sh',
            'src/shell/methods/*.sh',
            'src/shell/options.sh',
            'src/shell/run.sh'
          ],
          dest: 'bin/imageOptim'
        }, {
          src: 'src/md/README.md',
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
      Object.keys(data).forEach(function(token){
        contents = contents.replace(new RegExp('\\{\\{' + token + '\\}\\}', 'g'), data[token]);
      });
      writeFile(filePath, contents, onComplete);
    });
  }

  function lineToShellEcho (line) {
    return '  echo "' + line + '"';
  }

  function lineToMarkdownCodeBlock (line) {
    return '    ' + line;
  }

  grunt.registerMultiTask('environment', 'Apply environment config to build', function() {

    var task = this;
    var taskComplete = task.async();

    // @TODO: DRY this mess up
    // @TODO: use promises

    task.data.version = require('./package.json').version;

    readFile('src/txt/usage.txt', function(usage) {

      task.data.usage = usage.split('\n').map(lineToShellEcho).join('\n');

      readFile('src/md/examples.md', function(examples) {

        task.data.examples = examples.split('\n').map(lineToShellEcho).join('\n');

        mergeObjectWithFile('bin/imageOptim', task.data, function() {
          mergeObjectWithFile('bin/imageOptimAppleScriptLib', task.data, function() {

            task.data.usage = usage.split('\n').map(lineToMarkdownCodeBlock).join('\n');
            task.data.examples = examples;

            mergeObjectWithFile('README.md', task.data, function() {
              taskComplete();
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

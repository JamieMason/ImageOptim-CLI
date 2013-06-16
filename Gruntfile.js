module.exports = function(grunt) {

  'use strict';

  grunt.initConfig({

    environment: {
      base: require('./environment/base.json')
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
            'src/shell/methods.sh',
            'src/shell/process_directory.sh',
            'src/shell/handle_terminal_input.sh',
            'src/shell/validate.sh',
            'src/shell/run_applications.sh'
          ],
          dest: 'bin/imageOptim'
        }]
      }
    }

  });

  grunt.registerMultiTask('environment', 'Apply environment config to build', function() {

    var fs = require('fs');
    var path = require('path');
    var task = this;
    var taskComplete = task.async();

    task.data.version = require('./package.json').version;

    function mergeConfig (filePath, onComplete) {
      filePath = path.resolve(filePath);
      fs.readFile(filePath, 'utf8', function (err, fileContents) {
        var token;
        if (err) {
          throw err;
        }
        for(token in task.data) {
          fileContents = fileContents.replace(new RegExp('\\{\\{' + token + '\\}\\}', 'g'), task.data[token]);
        }
        fs.writeFile(filePath, fileContents, function (err) {
          if (err) {
            throw err;
          }
          onComplete();
        });
      });
    }

    mergeConfig('bin/imageOptim', function () {
      mergeConfig('bin/imageOptimAppleScriptLib', function () {
        taskComplete();
      });
    });

  });

  grunt.loadNpmTasks('grunt-contrib-concat');

  grunt.registerTask('build', ['concat', 'environment']);

};

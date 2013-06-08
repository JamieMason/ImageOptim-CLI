module.exports = function(grunt) {

  'use strict';

  grunt.initConfig({

    concat: {
      dist: {
        files: [{
          src: [
            'src/applescript/file_header.txt',
            'src/applescript/copyright.txt',
            'src/applescript/has_app_installed.applescript',
            'src/applescript/has_gui_script.applescript',
            'src/applescript/open_accessibility_preferences.applescript',
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
            'src/shell/handle_terminal_input.sh',
            'src/shell/validate.sh',
            'src/shell/run_applications.sh'
          ],
          dest: 'bin/imageOptim'
        }]
      }
    }

  });

  grunt.loadNpmTasks('grunt-contrib-concat');

  // grunt.registerTask('test', ['imageoptim', 'nodeunit']);

};

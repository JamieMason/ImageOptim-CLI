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
          src: 'src/imageOptimBashLib',
          dest: 'bin/imageOptimBashLib'
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

  grunt.loadTasks('tasks');

  grunt.loadNpmTasks('grunt-contrib-concat');
  grunt.loadNpmTasks('grunt-contrib-watch');

  grunt.registerTask('build', ['concat', 'environment']);

};

module.exports = function(grunt) {

  'use strict';

  grunt.initConfig({

    imageoptim: {
      files: [
        '../images/imageoptim-cli'
      ],
      options: {
        imageAlpha: true,
        jpegMini: false,
        quitAfter: true
      }
    },

    imageoptim_jpegmini: {
      files: [
        '../images/imageoptim-cli-jpegmini'
      ],
      options: {
        imageAlpha: true,
        jpegMini: true,
        quitAfter: true
      }
    },

    imagemin: {
      dist: {
        options: {
          // 240 trials
          optimizationLevel: 7
        },
        files: [{
            expand: true,
            cwd: '../images/',
            src: ['grunt-contrib-imagemin/*'],
            dest: '../images/'
          }
        ]
      }
    },

    cssmin: {
      browser: {
        files: {
          'browser/min.css': ['browser/styles.css']
        }
      }
    },

    transform: {
      results: {
        src: 'raw-results.json',
        dest: 'browser/results.js'
      }
    },

    uglify: {
      browser: {
        options: {
          beautify: false
        },
        files: {
          'browser/min.js': [
            'browser/results.js',
            'browser/angular-1.0.7.min.js',
            'browser/app.js'
          ]
        }
      }
    },

    jade: {
      index: {
        options: {
          pretty: false
        },
        files: {
          '../index.html': ['index.jade']
        }
      }
    }

  });

  grunt.loadNpmTasks('grunt-contrib-cssmin');
  grunt.loadNpmTasks('grunt-contrib-imagemin');
  grunt.loadNpmTasks('grunt-contrib-jade');
  grunt.loadNpmTasks('grunt-contrib-uglify');
  grunt.loadNpmTasks('grunt-imageoptim');

  grunt.task.loadTasks('tasks');

  grunt.task.registerTask('build', ['uglify', 'cssmin', 'jade']);
  grunt.task.registerTask('data', ['gather', 'transform']);

};

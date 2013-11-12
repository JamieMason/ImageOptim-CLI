module.exports = function(grunt) {

  'use strict';

  var _ = require('lodash');
  var debug = false;
  var results = require('./json/transformed-results.json');
  var baseJadeOptions = {
    pretty: debug,
    data: {
      sortBy: 'image',
      sortDirection: 'asc',
      sortDirectionToggle: 'desc',
      filter: 'all',
      results: results,
      tools: tools
    }
  };

  var tools = {
    'codekit': 'CodeKit',
    'grunt-contrib-imagemin': 'Grunt',
    'imageoptim': 'Optim',
    'imagealpha-and-imageoptim': 'Optim/IA',
    'jpegmini-and-imageoptim': 'Optim/JM',
    'kraken': 'Kraken',
    'photoshop': 'Photoshop',
    'smushit': 'Smushit',
    'tinypng': 'TinyPNG'
  };
  var filters = ['all', 'jpeg', 'png', 'gif'];
  var sortDirections = ['asc', 'desc'];
  var toolNames = Object.keys(tools);

  var documents = filters.reduce(function(memo, filter) {
    toolNames.forEach(function(toolName) {
      sortDirections.forEach(function(sortDirection) {
        var documentName = filter + '/' + toolName + '/' + sortDirection;
        var files = {};
        var filteredResults = filter !== 'all' ? _.filter(results, function(el) {
          return el.image.indexOf(filter) !== -1;
        }) : results;
        var sortedResults = _.sortBy(filteredResults, function(el) {
          return toolName === 'photoshop' ? el.size : el[toolName].size;
        });
        files['../comparison/' + documentName + '/index.html'] = ['jade/result-set.jade'];
        memo[documentName] = {
          files: files,
          options: {
            pretty: debug,
            data: {
              sortBy: toolName,
              sortDirection: sortDirection,
              sortDirectionToggle: sortDirection === 'desc' ? 'asc' : 'desc',
              filter: filter,
              results: sortDirection === 'desc' ? sortedResults.reverse() : sortedResults,
              tools: tools
            }
          }
        };
      });
    });
    return memo;
  }, {});

  grunt.initConfig({

    // imageoptim: {
    //   files: [
    //     '../images/imageoptim'
    //   ],
    //   options: {
    //     imageAlpha: false,
    //     jpegMini: false,
    //     quitAfter: true
    //   }
    // },

    // imageoptim: {
    //   files: [
    //     '../images/imageoptim_imagealpha'
    //   ],
    //   options: {
    //     imageAlpha: true,
    //     jpegMini: false,
    //     quitAfter: true
    //   }
    // },

    imageoptim: {
      files: [
        '../images/imageoptim_jpegmini'
      ],
      options: {
        imageAlpha: false,
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
        }]
      }
    },

    cssmin: {
      browser: {
        files: {
          'css/styles.min.css': ['css/styles.css']
        }
      }
    },

    transform: {
      results: {
        src: 'json/raw-results.json',
        dest: 'json/transformed-results.json'
      }
    },

    jade: documents,

    shell: {
      chooseIndex: {
        command: 'cp ../comparison/all/photoshop/desc/index.html ../index.html'
      }
    },

    watch: {
      scripts: {
        files: ['jade/*.jade'],
        tasks: ['build'],
        options: {
          nospawn: true
        }
      }
    }

  });

  grunt.loadNpmTasks('grunt-contrib-cssmin');
  grunt.loadNpmTasks('grunt-contrib-imagemin');
  grunt.loadNpmTasks('grunt-contrib-jade');
  grunt.loadNpmTasks('grunt-contrib-watch');
  grunt.loadNpmTasks('grunt-imageoptim');
  grunt.loadNpmTasks('grunt-shell');

  grunt.task.loadTasks('tasks');

  grunt.task.registerTask('build', [
    'cssmin',
    'jade',
    'shell'
  ]);

  grunt.task.registerTask('data', [
    'gather',
    'transform'
  ]);

};

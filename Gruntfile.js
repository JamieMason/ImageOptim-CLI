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
    },

    shell: {
      bashTests: {
        command: './test/imageOptimBashLib_test',
        options: {
          callback: function(err, stdout) {
            return err ? grunt.fatal(stdout) : grunt.log.write(stdout);
          }
        }
      }
    }

  });

  grunt.loadTasks('tasks');

  grunt.loadNpmTasks('grunt-contrib-concat');
  grunt.loadNpmTasks('grunt-contrib-watch');
  grunt.loadNpmTasks('grunt-shell');

  grunt.registerTask('build', ['concat', 'environment']);
  grunt.registerTask('test', ['shell:bashTests']);

};

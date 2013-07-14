angular.module('AssessCompress', []).controller('AppCtrl', ['$scope',
  function($scope) {

    'use strict';

    var results = window.results;
    var filterCache = {};

    $scope.results = results.all;
    $scope.total = results.total.all;
    $scope.ordering = {
      key: 'image',
      descending: true,
      filter: ''
    };

    $scope.orderBy = function(key) {
      $scope.ordering.key = key;
      $scope.ordering.descending = !$scope.ordering.descending;
    };

    $scope.css = function(result, key) {
      var classes = [];
      var best = ' ' + (result.best || []).join(' ') + ' ';

      if (best.indexOf(' ' + key + ' ') !== -1) {
        classes.push('best');
      }

      if (result['diff_' + key] === 0) {
        classes.push('noop');
      }

      return classes.join(' ');
    };

    $scope.text = function(result, key) {
      return result['size_' + key] + ' (' + result['saving_' + key] + '%)';
    };

    $scope.$watch('ordering.filter', function(filter, prev) {

      var i = 0;
      var len = results.all.length;

      if (filter === prev) {
        return;
      }

      if (!filter) {
        $scope.results = results.all;
        $scope.total = results.total.all;
      } else {
        $scope.total = results.total[filter];
        $scope.results = [];

        for (i = 0; i < len; i++) {
          if (results.all[i].image.indexOf(filter) !== -1) {
            $scope.results.push(results.all[i]);
          }
        }
      }

    });

  }
]);

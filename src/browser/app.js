angular.module('AssessCompress', [])

.run(function() {
  angular.element(document).bind('click', function(e) {
    if (angular.element(e.target).attr('href') === '#') {
      e.stopPropagation();
      e.preventDefault();
    }
  });
})

.controller('AppCtrl', ['$scope', function($scope) {

    'use strict';

    var results = window.results;

    $scope.results = results;
    $scope.ordering = {
      key: 'size',
      descending: true,
      filter: ''
    };

    $scope.orderBy = function(key) {
      $scope.ordering.key = key;
      $scope.ordering.descending = !$scope.ordering.descending;
    };

    $scope.css = function(result, key) {
      var classes = [];
      var sizeLoss = result[key].sizeLoss;

      if (result[key].isSmallest === true) {
        classes.push('best');
      }

      if (sizeLoss === 0) {
        classes.push('noop');
      } else if (sizeLoss === 'N/A') {
        classes.push('na');
      } else if (sizeLoss < 0) {
        classes.push('degrade');
      }

      return classes.join(' ');
    };

    $scope.text = function(result, key) {
      var size = result[key].size;
      return size === 'N/A' ? 'N/A' : size + '<br>(' + result[key].sizeLossPercent + '%) / ' + result[key].qualityLossPercent + '%';
    };

    $scope.$watch('ordering.filter', function(filter, prev) {

      var i = 0;
      var len = results.length;

      if (filter === prev) {
        return;
      }

      if (!filter) {
        $scope.results = results;
      } else {
        $scope.results = [];

        for (i = 0; i < len; i++) {
          if (results[i].image.indexOf(filter) !== -1) {
            $scope.results.push(results[i]);
          }
        }
      }

    });

  }
])

.directive('result', function factory() {
  var template = '';

  template += '<div>';
  template += '  <div ng-if="!isNa">';
  template += '    <a ng-href="images/{{tool}}/{{image}}" class="result">{{result.size}} bytes</a>';
  template += '    <br>';
  template += '    {{result.sizeLossPercent}}% saving';
  template += '    <span ng-if="result.qualityLossPercent > 0"><br>{{result.qualityLossPercent}}% loss</span>';
  template += '  </div>';
  template += '  <span ng-if="isNa">N/A</span>';
  template += '</div>';

  return {
    template: template,
    replace: true,
    restrict: 'A',
    scope: {
      result: '=',
      tool: '=',
      image: '='
    },
    link: function postLink(scope, iElement, iAttrs) {
      scope.isNa = scope.result.size === 'N/A';
    }
  };
});

require('file-loader?name=[name].[ext]!./index.html');
import _ from 'lodash';
import './style.css';

  function component() {
    const element = document.createElement('div');

    element.innerHTML = _.join(['Hello', 'webpack'], ' ');

    return element;
  }

  document.body.appendChild(component());
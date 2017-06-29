import React, { Component } from 'react';
import './App.css';

import AppHeader from './components/AppHeader.js';
import AppBody from './components/app-body/AppBody.js';

class App extends Component {
  render() {
    return (
      <div className="App">
        <AppHeader />
        <AppBody />
      </div>
    );
  }
}

export default App;

import React, { Component } from 'react';
import '../../App.css';
import SearchBar from './SearchBar.js';
import InfoBody from './info-body/InfoBody.js';

class AppBody extends Component {
  render() {
    return (
      <div className="App-body">
        <SearchBar />
        <InfoBody />
      </div>
    );
  }
}

export default AppBody;

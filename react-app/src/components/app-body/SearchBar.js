import React, { Component } from 'react';
import '../../App.css';

class SearchBar extends Component {
  render() {
    return (
      <div className="search-bar">
        <input className="search-input" type="text" placeholder="nick 0, nick 1, nick 2, ..."></input>
      </div>
    );
  }
}

export default SearchBar;

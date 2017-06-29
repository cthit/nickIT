import React, { Component } from 'react';
import '../../App.css';

class SearchBar extends Component {

  constructor() {
    super();

    this.previous_input = "";
  }

  render() {
    return (
      <div className="search-bar">
        <input id="search-input" className="search-input" type="text" placeholder="nick 0, nick 1, nick 2, ..." onKeyUp={this.search.bind(this)}></input>
      </div>
    );
  }

  search() {
    var raw_input = document.getElementById('search-input').value;
    this.previous_input = raw_input;
    // send raw_input to Flask here!
  }

}

export default SearchBar;

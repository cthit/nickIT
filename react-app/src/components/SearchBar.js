import React, { Component } from 'react';
import '../App.css';
let request = require('superagent');

class SearchBar extends Component {

  constructor() {
    super();

    this.previous_input = "";
  }

  request() {
    request
      .get("http://localhost:5000/search/" + this.previous_input)
      .accept('json')
      .end( (err, res) => {
        console.log('err: ' + err);
        if (res) {
          console.log(JSON.parse(res.text));
        }
      });
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
    this.request();
    //console.log("i did stuff");
    // send raw_input to Flask here!
  }

}

export default SearchBar;

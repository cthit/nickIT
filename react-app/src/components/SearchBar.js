import React, { Component } from "react";
import "../App.css";
let request = require("superagent");

class SearchBar extends Component {
  constructor() {
    super();
    this.previous_input = "";
  }

  request() {
    request
      .get("http://localhost:8000/search/nick/" + this.previous_input)
      .accept("json")
      .end((err, res) => this.handle_response(err, res));
  }

  handle_response(error, response) {
    if (error) {
    }
    if (response) {
      if (response.status === 400) {
        this.props.onSearch([]);
      } else if (response.text === "") {
        this.props.onSearch(null);
      } else {
        try {
          var responseObject = JSON.parse(response.text);
          console.log(responseObject);
          this.props.onSearch(responseObject);
        } catch (Error) {
          // Isn't JSON so we do nothing with the response.
        }
      }
    }
  }

  render() {
    return (
      <div className="search-bar">
        <input
          id="search-input"
          className="search-input"
          type="text"
          placeholder="nick 0, nick 1, ..."
          onChange={this.search.bind(this)}
        />
      </div>
    );
  }

  search() {
    var raw_input = document.getElementById("search-input").value;
    this.previous_input = raw_input;
    this.request();
  }
}

export default SearchBar;

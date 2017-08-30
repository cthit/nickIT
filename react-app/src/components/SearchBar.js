import React, { Component } from "react";
import "../App.css";
import request from "superagent";
const baseurl = process.env.REACT_APP_API_HOST + "/search/nick/";

class SearchBar extends Component {
  constructor() {
    super();
    this.previous_input = "";
  }

  tokenize_query(query) {
    let params = query.split(",");
    params = params.map(p => p.trim());
    params = params.filter(p => p !== "");
    return JSON.stringify(params);
  }

  request() {
    request
      .get(baseurl + this.tokenize_query(this.previous_input))
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

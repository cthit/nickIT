import React, { Component } from "react";
import "../App.css";
import request from "superagent";
const baseurl = process.env.REACT_APP_API_HOST + "/search/nick/";

class SearchBar extends Component {
  state = {
    previous_input: ""
  };

  tokenize_query(query) {
    let params = query.split(",");
    params = params.map(p => p.trim());
    params = params.filter(p => p !== "");
    return JSON.stringify(params);
  }

  request = () => {
    let previous_input = this.state.previous_input;
    request
      .get(baseurl + this.tokenize_query(previous_input))
      .end((err, res) => this.handle_response(err, res));
  };

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

  search = value => {
    this.setState(
      {
        previous_input: value
      },
      this.request
    );
  };

  render() {
    return (
      <div className="search-bar">
        <input
          id="search-input"
          className="search-input"
          type="search"
          placeholder="nick 0, nick 1, ..."
          onChange={e => this.search(e.target.value)}
        />
      </div>
    );
  }
}

export default SearchBar;

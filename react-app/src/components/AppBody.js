import React, { Component } from 'react';
import '../App.css';
import SearchBar from './SearchBar.js';
import InfoBody from './InfoBody.js';

class AppBody extends Component {

  constructor() {
    super();
    this.state = {
      nick_list: []
    };
  }

  render() {
    return (
      <div className="App-body">
        <SearchBar onSearch={this.handleSearch.bind(this)}/>
        <InfoBody nick_list={this.state.nick_list}/>
      </div>
    );
  }

  handleSearch(list) {
    this.setState({
      nick_list: list
    });
  }
}

export default AppBody;

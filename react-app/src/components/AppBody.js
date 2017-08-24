import React, { Component } from "react";
import "../App.css";
import SearchBar from "./SearchBar.js";
import NickList from "./NickList";
import StatisticsContainer from "./StatisticsContainer";

class AppBody extends Component {
  constructor() {
    super();
    this.state = {
      nick_list: []
    };
  }

  render() {
    const { nick_list: NICK_LIST } = this.state;
    const handleSearch = this.handleSearch.bind(this);

    return (
      <div className="app-body">
        <SearchBar onSearch={handleSearch} />
        <main className="info-body">
          <NickList nick_list={NICK_LIST} />
          <StatisticsContainer />
        </main>
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

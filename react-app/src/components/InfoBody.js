import React, { Component } from "react";
import "../App.css";
import NickList from "./NickList";
import StatisticsContainer from "./StatisticsContainer";

class InfoBody extends Component {
  render() {
    return (
      <main className="Info-body">
        <NickList nick_list={this.props.nick_list} />
        <StatisticsContainer />
      </main>
    );
  }
}

export default InfoBody;

import React, { Component } from "react";
import "../App.css";
import NickList from "./NickList";
import StatisticsContainer from "./StatisticsContainer";

class InfoBody extends Component {
  render() {
    return (
      <div className="Info-body">
        <NickList nick_list={this.props.nick_list} />
        <StatisticsContainer />
      </div>
    );
  }
}

export default InfoBody;

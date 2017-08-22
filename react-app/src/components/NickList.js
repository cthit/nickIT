import React, { Component } from "react";
import "../App.css";

class NickList extends Component {
  render() {
    const { nick_list } = this.props;

    return (
      <div className="Nick-list">
        <h2 className="nick-list-heading">Nick list</h2>
        {this.getNickList(nick_list)}
      </div>
    );
  }

  getNickList(nick_list) {
    var listItems = nick_list.map(item =>
      <li key={item.uid} className="nick-item">
        {item.nick}
      </li>
    );
    return (
      <ul className="nick-list">
        {listItems}
      </ul>
    );
  }
}

export default NickList;

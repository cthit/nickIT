import React, { Component } from 'react';
import '../App.css';

class NickList extends Component {

  render() {
    return (
      <div className="Nick-list">
        <p>Nick list</p>
        {this.getNickList(this.props.nick_list)}
      </div>
    );
  }

  getNickList(nick_list) {
    var listItems = nick_list.map( nick =>
      <li>{nick}</li>
    );
    return <ul>{listItems}</ul>;
  }

}

export default NickList;

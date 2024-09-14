var React = require("react");

module.exports = class extends React.Component {
  render() {
    return (
      <div>
        <For each="otherItem" of={this.props.otherItems}>
          <For each="item" of={this.props.items}>
            <span key={otherItem + item}>{otherItem + item}</span>
          </For>
        </For>
        {this.props.otherItems.map(function(otherItem){
          return this.props.items.map(function(item){
            return <span key={otherItem + item}>{otherItem + item}</span>;
          });
        })}
      </div>
    );
  }
};

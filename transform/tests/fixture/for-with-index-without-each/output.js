var React = require("react");

module.exports = class extends React.Component {
  render() {
    return (
      <div>
        {this.props.items.map(function(item, $index){
          return <span key={$index}>{$index}</span>;
        })}
      </div>
    );
  }
};

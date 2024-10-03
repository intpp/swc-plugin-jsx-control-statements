var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>
        {this.props.otherItems.map(function(otherItem, $index1){
            return this.props.items.map(function(item, $index2){
                return <span key={otherItem + item}>{otherItem + item + $index2 + $index1}</span>;
            }, this);
        }, this)}
      </div>;
    }
};

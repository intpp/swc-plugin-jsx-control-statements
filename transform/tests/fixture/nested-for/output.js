var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>
        {this.props.otherItems.map(function(otherItem, _) {
            return this.props.items.map(function(item, _) {
                return <span key={otherItem + item}>{otherItem + item}</span>;
            }, this);
        }, this)}
      </div>;
    }
};

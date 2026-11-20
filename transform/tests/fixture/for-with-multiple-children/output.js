var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>
        {this.props.items.map(function(item) {
            return [
                <span key={item}>{item}</span>,
                <span key={item + "test"}>{item + "test"}</span>
            ];
        }, this)}
      </div>;
    }
};
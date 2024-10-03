var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>
        {this.props.test ? this.props.blahs.map(function(blah, _) {
            return <span key={blah}>{blah}</span>;
        }, this) : this.props.otherBlahs.map(function(otherBlah, _) {
            return <span key={otherBlah}>{otherBlah}</span>;
        }, this)}
      </div>;
    }
};
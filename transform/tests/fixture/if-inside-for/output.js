var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>
        {this.props.blahs.map(function(blah, _) {
            return blah === "blah1" ? <span key={blah}>{blah}</span> : null;
        }, this)}
      </div>;
    }
};
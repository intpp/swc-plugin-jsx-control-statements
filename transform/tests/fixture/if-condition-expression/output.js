var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.condition === "blah" ? <span>IfBlock</span> : null}

            </div>;
    }
};

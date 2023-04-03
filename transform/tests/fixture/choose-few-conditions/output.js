var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.when1 ? <span>WhenBlock1</span> : this.props.when2 ? <span>WhenBlock2</span> : null}

            </div>;
    }
};

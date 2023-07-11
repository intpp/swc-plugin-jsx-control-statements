var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.ifCondition ? [
            <span key="prefix-0">test</span>,
            <span key="prefix-1">test</span>,
            <span key="prefix-2">test</span>,
            <span key="prefix-3">test</span>,
            <span key="prefix-4">test</span>
        ] : null}

            </div>;
    }
};

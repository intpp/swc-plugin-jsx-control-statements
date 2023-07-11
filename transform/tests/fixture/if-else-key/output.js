var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.ifCondition ? [
            <span key="custom-0">test</span>,
            <span key="custom-1">test</span>,
            <span key="custom-2">test</span>
        ] : [
            <span key="custom-0">test</span>,
            <span key="custom-1">test</span>,
            <span key="custom-2">test</span>
        ]}

            </div>;
    }
};

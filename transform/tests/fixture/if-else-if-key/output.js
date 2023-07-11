var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.ifCondition ? true ? [
            <span key="super-0">test</span>,
            <span key="super-1">test</span>
        ] : [
            <span key="super-0">test</span>,
            <span key="super-1">test</span>
        ] : [
            <span key="super-0">test</span>,
            <span key="super-1">test</span>,
            <span key="super-2">test</span>
        ]}

            </div>;
    }
};

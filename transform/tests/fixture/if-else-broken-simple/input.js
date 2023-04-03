var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If condition={this.props.condition === "blah"}>
                    <span>left-side</span>
                    <Else>
                    <span>right-side</span>
                    </Else>
                </If>
            </div>
        );
    }
};

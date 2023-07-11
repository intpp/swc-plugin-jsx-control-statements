var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If key="custom" condition={this.props.ifCondition}>
                    <span>test</span>
                    <span>test</span>
                    <span>test</span>
                    <Else />
                    <span>test</span>
                    <span>test</span>
                    <span>test</span>
                </If>
            </div>
        );
    }
};

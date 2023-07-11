var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If key="prefix" condition={this.props.ifCondition}>
                    <span>test</span>
                    <span>test</span>
                    <span>test</span>
                    <span>test</span>
                    <span>test</span>
                </If>
            </div>
        );
    }
};

var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If key="super" condition={this.props.ifCondition}>
                    <If condition={true}>
                        <span>test</span>
                        <span>test</span>
                    <Else />
                        <span>test</span>
                        <span>test</span>
                    </If>
                    <Else />
                    <span>test</span>
                    <span>test</span>
                    <span>test</span>
                </If>
            </div>
        );
    }
};

var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If condition={this.props.condition !== "world"}>
                    <If condition={true}><p>world</p><Else /><p>Hello</p></If>
                </If>
            </div>
        );
    }
};

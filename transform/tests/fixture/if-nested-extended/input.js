var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If condition={this.props.condition === "blah"}>
                    <span>
                        <If condition={true}>
                            <p>hello</p>
                        </If>

                        <If condition={this.props.condition !== "world"}>
                            <If condition={true}><p>world</p></If>
                        </If>
                    </span>
                </If>
            </div>
        );
    }
};

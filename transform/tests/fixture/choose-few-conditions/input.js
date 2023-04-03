var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <Choose>
                    <When condition={this.props.when1}>
                        <span>WhenBlock1</span>
                    </When>
                    <When condition={this.props.when2}>
                        <span>WhenBlock2</span>
                    </When>
                </Choose>
            </div>
        );
    }
};

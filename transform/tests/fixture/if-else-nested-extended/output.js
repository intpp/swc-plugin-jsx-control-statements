var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.ifCondition ? <><span>test</span>{this.props.nestedIfCondition ? <span>If-If</span> : <span>If-Else</span>}</> : <><span>test2</span>{this.props.nestedIfCondition ? <span>Else-If</span> : <span>Else-Else</span>}</>}

            </div>;
    }
};

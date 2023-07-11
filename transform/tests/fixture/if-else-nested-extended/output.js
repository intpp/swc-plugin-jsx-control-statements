var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.ifCondition ? [
            <span key="0">test</span>,
            this.props.nestedIfCondition ? <span key="1">If-If</span> : <span key="1">If-Else</span>
        ] : [
            <span key="0">test2</span>,
            this.props.nestedIfCondition ? <span key="1">Else-If</span> : <span key="1">Else-Else</span>
        ]}

            </div>;
    }
};
